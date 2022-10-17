use ethers_core::types::U256;

use crate::genes::{self, Profession::*};

pub type PoolId = U256;

pub enum QuestType {
    Profession(genes::Profession),
    _Training(genes::BaseStat),
}

pub fn team_size(quest_type: QuestType) -> usize {
    match quest_type {
        QuestType::Profession(Gardening) => 2,
        _ => 6,
    }
}
