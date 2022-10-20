use crate::genes::BaseStat;
use crate::quests::PoolId;
use confique::Config;
use ethers_core::types::Address;
use eyre::Result;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Config, Debug)]
pub struct Root {
    pub chain_id: u64,
    pub rpc_url: String,
    pub female_first_names: Vec<String>,
    pub male_first_names: Vec<String>,
    pub last_names: Vec<String>,

    #[config(nested)]
    pub core_contracts: CoreContracts,

    #[config(nested)]
    pub profession_quests: ProfessionQuests,

    #[config(nested)]
    pub stat_quests: StatQuests,

    // #[config(nested)]
    pub database: DatabaseConfig,
}

#[derive(Config, Debug)]
pub struct CoreContracts {
    pub hero_core: Address,
    pub quest_core: Address,
    pub meditation_circle: Address,
}

#[derive(Config, Debug)]
pub struct ProfessionQuests {
    pub crystal_mining: Address,
    pub gold_mining: Address,
    pub fishing: Address,
    pub foraging: Address,
    pub gardens: HashMap<PoolId, GardenQuestInfo>,
}

#[derive(Config, Debug)]
pub struct StatQuests {
    pub strength: Address,
    pub intelligence: Address,
    pub wisdom: Address,
    pub luck: Address,
    pub agility: Address,
    pub vitality: Address,
    pub endurance: Address,
    pub dexterity: Address,
}

#[derive(Deserialize, Debug)]
pub struct GardenQuestInfo {
    pub pool_id: PoolId,
    pub pair_name: String,
    pub lp_address: Address,
    pub quest_address: Address,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(tag = "driver")]
pub enum DatabaseConfig {
    SQLite { file: String },
}

impl Root {
    // TODO - this should consider explicit config or, more likely, staked gardens on chain
    pub fn active_garden_pool_id_for_quest_address(
        &self,
        quest_address: &Address,
    ) -> Option<PoolId> {
        for g in self.profession_quests.gardens.values() {
            if g.quest_address.eq(quest_address) {
                return Some(g.pool_id);
            }
        }

        return None;
    }

    pub fn address_for_stat_quest(&self, stat: BaseStat) -> Address {
        match stat {
            BaseStat::Strength => self.stat_quests.strength,
            BaseStat::Dexterity => self.stat_quests.dexterity,
            BaseStat::Agility => self.stat_quests.agility,
            BaseStat::Vitality => self.stat_quests.vitality,
            BaseStat::Endurance => self.stat_quests.endurance,
            BaseStat::Intelligence => self.stat_quests.intelligence,
            BaseStat::Wisdom => self.stat_quests.wisdom,
            BaseStat::Luck => self.stat_quests.luck,
        }
    }
}
