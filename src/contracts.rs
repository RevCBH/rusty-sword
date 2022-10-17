use crate::genes::BaseStat;

pub type Hero = hero_core::Hero;
pub type HeroCore<M> = hero_core::HeroCore<M>;
pub type QuestInfo = quest_core::QuestInfo;
pub type QuestCore<M> = quest_core::QuestCore<M>;
// pub type MeditationCircle<M> = meditation_circle::MeditationCircle<M>;

mod hero_core {
    use ethers_contract::abigen;
    abigen!(HeroCore, "artifacts/HeroCore.sol/HeroCore.json");
}

mod quest_core {
    use ethers_contract::abigen;
    abigen!(QuestCore, "artifacts/QuestCore.sol/QuestCoreV2_2.json");
}

mod meditation_circle {
    use ethers_contract::abigen;
    abigen!(
        MeditationCircle,
        "artifacts/MeditationCircle.sol/MeditationCircle.json"
    );
}

impl Hero {
    pub fn max_stat(&self) -> BaseStat {
        let mut max_val = self.stats.strength;
        let mut stat = BaseStat::Strength;

        if self.stats.dexterity > max_val {
            max_val = self.stats.dexterity;
            stat = BaseStat::Dexterity;
        }

        if self.stats.agility > max_val {
            max_val = self.stats.agility;
            stat = BaseStat::Agility;
        }

        if self.stats.vitality > max_val {
            max_val = self.stats.vitality;
            stat = BaseStat::Vitality;
        }

        if self.stats.endurance > max_val {
            max_val = self.stats.endurance;
            stat = BaseStat::Endurance;
        }

        if self.stats.intelligence > max_val {
            max_val = self.stats.intelligence;
            stat = BaseStat::Intelligence;
        }

        if self.stats.wisdom > max_val {
            max_val = self.stats.wisdom;
            stat = BaseStat::Wisdom;
        }

        if self.stats.luck > max_val {
            stat = BaseStat::Luck;
        }

        return stat;
    }
}
