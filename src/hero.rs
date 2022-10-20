use ethers_core::types::U256;

use crate::config;
use crate::contracts;
use crate::genes::{self, parse_stat_genes, BaseStat, StatTraits};
use eyre::{eyre, Result};

#[derive(sqlx::FromRow, Clone, Debug)]
pub struct Hero {
    pub id: U256,
    pub visual_genes: U256,
    pub stat_genes: U256,
    pub first_name: u32,
    pub last_name: u32,

    // stats
    pub strength: u16,
    pub dexterity: u16,
    pub agility: u16,
    pub vitality: u16,
    pub endurance: u16,
    pub intelligence: u16,
    pub wisdom: u16,
    pub luck: u16,

    // state
    pub xp: u64,
    pub level: u16,
}

impl From<&contracts::Hero> for Hero {
    fn from(h: &contracts::Hero) -> Self {
        Hero {
            id: h.id,
            visual_genes: h.info.visual_genes,
            stat_genes: h.info.stat_genes,
            first_name: h.info.first_name,
            last_name: h.info.last_name,
            strength: h.stats.strength,
            dexterity: h.stats.dexterity,
            agility: h.stats.agility,
            vitality: h.stats.vitality,
            endurance: h.stats.endurance,
            intelligence: h.stats.intelligence,
            wisdom: h.stats.wisdom,
            luck: h.stats.luck,
            xp: h.state.xp,
            level: h.state.level,
        }
    }
}

impl config::Root {
    pub fn hero_full_name(&self, hero: &Hero) -> Result<String> {
        let visual_traits = genes::parse_visual_genes(&hero.visual_genes)?;
        let gendered_names = match visual_traits.gender {
            genes::Gender::Male => &self.male_first_names,
            _ => &self.female_first_names,
        };

        let last = self
            .last_names
            .get(hero.last_name as usize)
            .ok_or(eyre!("bad last name index"))?;

        let first = gendered_names
            .get(hero.first_name as usize)
            .ok_or(eyre!("bad first name index"))?;

        Ok(format!("{} {}", first, last))
    }
}

impl Hero {
    pub fn stat_traits(&self) -> Result<StatTraits> {
        parse_stat_genes(&self.stat_genes)
    }

    pub fn max_stat(&self) -> BaseStat {
        let mut max_val = self.strength;
        let mut stat = BaseStat::Strength;

        if self.dexterity > max_val {
            max_val = self.dexterity;
            stat = BaseStat::Dexterity;
        }

        if self.agility > max_val {
            max_val = self.agility;
            stat = BaseStat::Agility;
        }

        if self.vitality > max_val {
            max_val = self.vitality;
            stat = BaseStat::Vitality;
        }

        if self.endurance > max_val {
            max_val = self.endurance;
            stat = BaseStat::Endurance;
        }

        if self.intelligence > max_val {
            max_val = self.intelligence;
            stat = BaseStat::Intelligence;
        }

        if self.wisdom > max_val {
            max_val = self.wisdom;
            stat = BaseStat::Wisdom;
        }

        if self.luck > max_val {
            stat = BaseStat::Luck;
        }

        return stat;
    }
}
