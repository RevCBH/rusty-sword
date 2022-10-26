#![feature(drain_filter)]
#![feature(async_closure)]

use clap::Parser;
use confique::Config;
use ethers_core::{
    k256::ecdsa::SigningKey,
    types::{Address, U256},
};
use ethers_middleware::signer::SignerMiddleware;
use ethers_providers::{Http, Middleware, Provider};
use ethers_signers::{LocalWallet, Signer, Wallet};
use eyre::{eyre, Result, WrapErr};
use rusty_sword::{
    command_line::{Cli, Commands},
    config,
    contracts::{HeroCore, QuestCore, QuestInfo},
    database,
    genes::{self, Profession},
    hero::Hero,
    level_up,
    quests::{self, QuestType},
};
use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteJournalMode},
    ConnectOptions, Connection,
};
use std::{collections::HashMap, convert::TryFrom, str::FromStr, sync::Arc};

type ProviderStack = SignerMiddleware<Provider<Http>, Wallet<SigningKey>>;

#[derive(Clone, Debug)]
enum HeroPath {
    Quest,
    Meditate,
}

#[derive(Debug)]
struct RunnerConfig<'a> {
    root_config: &'a config::Root,
    account_address: Address,
    hero_core: HeroCore<ProviderStack>,
    quest_core: Arc<QuestCore<ProviderStack>>,
}

#[derive(Clone, Debug)]
struct HeroPlan {
    hero: Hero,
    stamina: u16,
    path: HeroPath, //current_quest: Option<U256>,
}

#[derive(Debug)]
struct RunnerAnalysis<'a> {
    config: &'a RunnerConfig<'a>,

    // heros_by_id: HashMap<U256, Hero>,
    finished_quests: Vec<QuestInfo>,
    heros_ready_to_quest: Vec<HeroPlan>,
    // running_quests: Vec<QuestInfo>,
    is_mining_crystal: bool,
    is_mining_gold: bool,
    gardens_in_progress: Vec<quests::PoolId>,
}

fn is_quest_complete(now: u128) -> impl FnMut(&mut QuestInfo) -> bool {
    Box::new(move |q: &mut QuestInfo| q.complete_at_time.as_u128() < now)
}

struct QuestBatcher<'a, M: Middleware> {
    quest_addresses: Vec<Address>,
    hero_sets: Vec<Vec<U256>>,
    quest_attempts: Vec<u8>,
    quest_levels: Vec<u8>,
    quest_core: Arc<QuestCore<M>>,
    stat_sets: HashMap<genes::BaseStat, Vec<(u16, U256)>>,
    config: &'a config::Root,
}

impl<'a, M: 'static + Middleware> QuestBatcher<'a, M> {
    fn new(root_config: &config::Root, quest_core: Arc<QuestCore<M>>) -> QuestBatcher<M> {
        QuestBatcher {
            quest_addresses: vec![],
            hero_sets: vec![],
            quest_attempts: vec![],
            quest_levels: vec![],
            stat_sets: HashMap::new(),
            quest_core: quest_core,
            config: root_config,
        }
    }

    fn add(&mut self, quest_address: Address, hero_set: Vec<HeroPlan>, attempts: u8, level: u8) {
        self.add_ids(
            quest_address,
            hero_set.iter().map(|h| h.hero.id).collect(),
            attempts,
            level,
        )
    }

    fn add_ids(&mut self, quest_address: Address, hero_set: Vec<U256>, attempts: u8, level: u8) {
        println!(
            "queuing quest {} at level {} ({} heros, {} attempts)",
            quest_address,
            level,
            hero_set.len(),
            level
        );
        self.quest_addresses.push(quest_address);
        self.hero_sets.push(hero_set);
        self.quest_attempts.push(attempts);
        self.quest_levels.push(level);
    }

    fn queue_for_stats(&mut self, heros: Vec<HeroPlan>) {
        for h in heros {
            let max_stat = h.hero.max_stat();
            if !self.stat_sets.contains_key(&max_stat) {
                self.stat_sets.insert(max_stat, vec![]);
            }

            let mut x = self.stat_sets.get_mut(&max_stat).unwrap().clone();
            x.push((h.stamina, h.hero.id));

            if x.len() >= 6 {
                x.sort_by(|(a, _), (b, _)| a.cmp(b));
                let attempts = x[0].0 / 5;
                self.add_ids(
                    self.config.address_for_stat_quest(max_stat),
                    x.iter().map(|x| x.1).collect(),
                    attempts as u8,
                    1,
                );
                x.clear();
            }
            self.stat_sets.insert(max_stat, x);
        }
    }

    async fn try_send_batch<T: Into<U256>>(&mut self, gas_limit: T) -> Result<()> {
        let gas_limit = gas_limit.into();
        let call_start_quests = self.quest_core.multi_start_quest(
            self.quest_addresses.clone(),
            self.hero_sets.clone(),
            self.quest_attempts.clone(),
            self.quest_levels.clone(),
        );

        let gas_estimate = call_start_quests.estimate_gas().await?;
        if self.quest_addresses.len() > 0 && gas_estimate.gt(&gas_limit) {
            let tx = call_start_quests.send().await?;
            match tx.confirmations(1).await? {
                None => {
                    println!("failed to start quest batch");
                }
                Some(tx) => {
                    println!(
                        "start quest batch completed. gas used: {}",
                        tx.gas_used
                            .map_or(String::from("<unknown>"), |x| x.to_string())
                    );
                }
            }

            self.quest_addresses.clear();
            self.hero_sets.clear();
            self.quest_attempts.clear();
            self.quest_levels.clear();
        }

        Ok(())
    }

    async fn finish(&mut self) -> Result<()> {
        self.try_send_batch(0).await?;
        let x = self.stat_sets.clone();
        for (k, v) in x.iter() {
            let mut v = v.clone();
            v.sort_by(|(a, _), (b, _)| a.cmp(b));
            if v.len() == 0 {
                continue;
            }

            let attempts = v[0].0 / 5;
            let quest_address = self.config.address_for_stat_quest(*k);
            self.add_ids(
                quest_address,
                v.iter().map(|x| x.1).collect(),
                attempts as u8,
                1,
            );
        }

        self.try_send_batch(0).await
    }
}

impl<'a> RunnerConfig<'a> {
    fn new(wallet: Wallet<SigningKey>, conf: &config::Root) -> Result<RunnerConfig> {
        let account_address = wallet.address();

        let provider = Provider::<Http>::try_from(conf.rpc_url.as_str())
            .wrap_err("failed to init provider")?;
        let provider = SignerMiddleware::new(provider, wallet.clone());
        let client = Arc::new(provider);

        let qc = QuestCore::new(conf.core_contracts.quest_core, client.clone());
        let hc = HeroCore::new(conf.core_contracts.hero_core, client.clone());

        Ok(RunnerConfig {
            root_config: conf,
            account_address: account_address,
            hero_core: hc,
            quest_core: Arc::new(qc),
        })
    }

    async fn analyze(&self) -> Result<RunnerAnalysis> {
        // let contract_addresses = config::mainnet_contract_addresses()?;
        let call = self
            .quest_core
            .get_account_active_quests(self.account_address);
        let mut active_quests = call.call().await.wrap_err("failed to get active quests")?;

        let now = std::time::SystemTime::now();
        let timestamp = now.duration_since(std::time::UNIX_EPOCH)?.as_millis() / 1000;

        let finished_quests: Vec<QuestInfo> = active_quests
            .drain_filter(is_quest_complete(timestamp))
            .collect();

        println!("{} quests in progress", active_quests.len());
        println!("{} quests complete", finished_quests.len());

        let user_heros = self
            .hero_core
            .get_user_heroes(self.account_address)
            .call()
            .await?;

        let mut conn = database::connect_sqlite(&self.root_config.database.clone()).await?;
        let mut hero_datas = user_heros.iter().map(|id| self.gather_hero_data(*id));
        let mut questable_heros = vec![];
        let mut meditators = vec![];
        while let Some(d) = hero_datas.next() {
            match d.await {
                Some(plan) => match plan.path {
                    //  plan.hero
                    HeroPath::Quest => {
                        questable_heros.push(plan);
                    }
                    HeroPath::Meditate => {
                        meditators.push(plan);
                    }
                },
                None => (),
            };
        }

        let is_mining_crystal = active_quests.iter().any(|q| {
            q.quest_address
                .eq(&self.root_config.profession_quests.crystal_mining)
        });

        let is_mining_gold = active_quests.iter().any(|q| {
            q.quest_address
                .eq(&self.root_config.profession_quests.gold_mining)
        });

        let active_gardens = active_quests
            .iter()
            .filter_map(|q| {
                self.root_config
                    .active_garden_pool_id_for_quest_address(&q.quest_address)
            })
            .collect::<Vec<_>>();
        println!("currently questing gardens: {:?}", active_gardens);

        Ok(RunnerAnalysis {
            config: self,
            finished_quests: finished_quests,
            heros_ready_to_quest: questable_heros,
            gardens_in_progress: active_gardens,
            is_mining_crystal: is_mining_crystal,
            is_mining_gold: is_mining_gold,
        })
    }

    async fn gather_hero_data(&self, id: U256) -> Option<HeroPlan> {
        let get_hero = self.hero_core.get_hero(id);
        let get_stamina = self.quest_core.get_current_stamina(id);
        let get_quest = self.quest_core.get_hero_quest(id);

        let (hero_info, hero_stamina, hero_current_quest) =
            tokio::join!(get_hero.call(), get_stamina.call(), get_quest.call());

        let hero_current_quest = hero_current_quest.ok().and_then(|q| {
            if q.id == U256::from(0) {
                None
            } else {
                Some(q.id)
            }
        });

        let hero = Hero::from(&hero_info.ok()?);
        let stamina = hero_stamina.ok()?.try_into().ok()?;

        // println!("{} {}", first_name, last_name);
        // let h2 = Hero::from(&hero);
        // println!("{:?}", self.root_config.hero_full_name(&h2));

        // println!(
        //     "hero {} xp: {}/{:?}",
        //     hero.id,
        //     hero.state.xp,
        //     level_up::xp_to_level(hero.state.level)
        // );

        let path = if hero_current_quest.is_none() {
            if stamina >= 15 {
                Some(HeroPath::Quest)
            } else {
                level_up::xp_to_level(hero.level).and_then(|required_xp| {
                    if hero.xp == required_xp {
                        Some(HeroPath::Meditate)
                    } else {
                        None
                    }
                })
            }
        } else {
            None
        };

        Some(HeroPlan {
            hero: hero,
            stamina: stamina,
            path: path?,
        })
    }
}

#[derive(PartialOrd, Copy, Clone, Ord, PartialEq, Eq, Hash, Debug)]
struct QuestCohort(u8, Profession);

impl RunnerAnalysis<'_> {
    async fn complete_quests(&self) -> Result<()> {
        let quest_core = &self.config.quest_core;

        let now = std::time::SystemTime::now();
        let timestamp = now.duration_since(std::time::UNIX_EPOCH)?.as_millis() / 1000;
        println!("now: {}", timestamp);

        let mut quests_to_finish: Vec<(U256, U256)> = vec![];
        for q in self.finished_quests.clone() {
            if q.complete_at_time.as_u128() < timestamp {
                println!(
                    "quest {} complete at {}",
                    q.id,
                    q.complete_at_time.as_u128()
                );
                let leader = q
                    .heroes
                    .first()
                    .ok_or(eyre!("missing leader for quest {}", q.id))?;

                let gas_estimate = quest_core.complete_quest(*leader).estimate_gas().await?;
                println!("gas estimate: {}", gas_estimate);
                quests_to_finish.push((*leader, gas_estimate));
            }
        }

        let res = self.complete_quest_batch(&quests_to_finish).await;
        match res {
            Ok(_) => Ok(()),
            Err(e) => {
                println!(
                    "error sending batch: {}\nfalling back to individual sends",
                    e
                );
                for q in quests_to_finish {
                    let call = quest_core.complete_quest(q.0);
                    let tx = call.send().await?;
                    tx.await?;
                    println!("completed quest led by: {}", q.0)
                }
                Ok(())
            }
        }
    }

    async fn complete_quest_batch(&self, quests_to_finish: &Vec<(U256, U256)>) -> Result<()> {
        let mut quests_to_finish = quests_to_finish.clone();
        let mut quest_batch: Vec<U256> = vec![];
        let mut total_gas_estimate: u128 = 0;

        let process_batch = |batch| async {
            println!("completing quests with hero ids: {:?}", batch);
            let call = self.config.quest_core.multi_complete_quest(batch);
            let tx = call.send().await?;
            match tx.confirmations(2).await? {
                None => {
                    println!("failed to complete quest batch");
                }
                Some(tx) => {
                    println!(
                        "quest batch completed. gas used: {}",
                        tx.gas_used
                            .map_or(String::from("<unknown>"), |x| x.to_string())
                    );
                }
            }

            Result::<()>::Ok(())
        };

        while !quests_to_finish.is_empty() {
            let (leader, gas_estimate) = quests_to_finish.pop().unwrap();
            let gas_estimate: u128 =
                u128::try_from(gas_estimate).map_err(|e| eyre!("gas estimate overflow! {}", e))?;

            // TODO - don't hardcode gas limit
            if total_gas_estimate + gas_estimate > 30000000 {
                process_batch(quest_batch.clone()).await?;

                total_gas_estimate = 0;
                quest_batch = vec![];
            }

            total_gas_estimate += gas_estimate;
            quest_batch.push(leader);
        }

        if !quest_batch.is_empty() {
            process_batch(quest_batch.clone()).await?;
        }

        Ok(())
    }

    async fn start_quests(&mut self) -> Result<()> {
        println!(
            "starting quests. {} heros ready.",
            self.heros_ready_to_quest.len()
        );

        let mut cohorts: HashMap<QuestCohort, Vec<HeroPlan>> = HashMap::new();
        let mut quest_queue: Vec<(QuestCohort, Vec<HeroPlan>)> = vec![];

        for h in self.heros_ready_to_quest.clone() {
            let profession = h.hero.stat_traits()?.profession;
            let s = (h.stamina / 5) as u8;
            // TODO - make this a config threshold
            if s >= 3 {
                let cohort = QuestCohort(s, profession);
                match cohorts.get_mut(&cohort) {
                    Some(hero_list) => {
                        hero_list.push(h);

                        let team_size = quests::team_size(QuestType::Profession(profession));
                        if hero_list.len() == team_size {
                            quest_queue.push((cohort, hero_list.clone()));
                            hero_list.clear();
                        }
                    }
                    None => {
                        cohorts.insert(cohort, vec![h]);
                    }
                }
            }
        }

        for (k, v) in cohorts.iter() {
            if v.len() > 0 {
                quest_queue.push((*k, v.clone()));
            }
        }

        let mut ready_gardens = self
            .config
            .root_config
            .profession_quests
            .gardens
            .iter()
            .filter_map(|(_, g)| {
                if !self.gardens_in_progress.contains(&g.pool_id) {
                    Some(g)
                } else {
                    None
                }
            });

        let mut batch = QuestBatcher::new(self.config.root_config, self.config.quest_core.clone());
        let mut current_garden = ready_gardens.next();

        for (QuestCohort(attempts, profession), hero_set) in quest_queue {
            match profession {
                Profession::Gardening => match current_garden {
                    Some(g) => {
                        batch.add(g.quest_address, hero_set, 1, 0);
                        current_garden = ready_gardens.next();
                    }
                    None => {
                        batch.queue_for_stats(hero_set);
                    }
                },
                Profession::Mining => {
                    if !self.is_mining_crystal {
                        batch.add(
                            self.config.root_config.profession_quests.crystal_mining,
                            hero_set,
                            1,
                            0,
                        );
                        self.is_mining_crystal = true;
                    } else if !self.is_mining_gold {
                        batch.add(
                            self.config.root_config.profession_quests.gold_mining,
                            hero_set,
                            1,
                            0,
                        );
                        self.is_mining_gold = true;
                    } else {
                        batch.queue_for_stats(hero_set);
                    }
                }
                Profession::Fishing => {
                    batch.add(
                        self.config.root_config.profession_quests.fishing,
                        hero_set,
                        attempts,
                        0,
                    );
                }
                Profession::Foraging => {
                    batch.add(
                        self.config.root_config.profession_quests.foraging,
                        hero_set,
                        attempts,
                        0,
                    );
                }
            }
            // TODO - make configurable? keep old state to check if we're going over 30M?
            batch.try_send_batch(21000000).await?
        }

        batch.finish().await?;

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let conf = config::Root::builder()
        .file("config/dfkchain/mainnet.toml")
        .file("config/hero_names.toml")
        .file("config/db.toml")
        .load()?;

    let mut conn = match conf.database.clone() {
        config::DatabaseConfig::SQLite { file } => {
            SqliteConnectOptions::from_str(file.as_str())?
                .journal_mode(SqliteJournalMode::Wal)
                .create_if_missing(true)
                .read_only(false)
                .connect()
                .await?
        }
    };

    sqlx::migrate!("db/migrations").run(&mut conn).await?;
    conn.close().await?;

    // let cli = Cli::parse();

    // match &cli.command {
    //     Commands::Scan => {
    //         println!("scanning!");
    //         return Ok(());
    //     }
    // }

    let wallet = std::env::var("HARMONY_PRIVATE_KEY")?
        .parse::<LocalWallet>()?
        .with_chain_id(conf.chain_id);

    let runner = RunnerConfig::new(wallet, &conf)?;
    let mut runner = runner.analyze().await?;

    runner.complete_quests().await?;
    runner.start_quests().await?;

    Ok(())
}
