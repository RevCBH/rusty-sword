use crate::genes::BaseStat;

pub type Hero = hero_core::Hero;
pub type HeroCore<M> = hero_core::HeroCore<M>;
pub type QuestInfo = quest_core::QuestInfo;
pub type QuestCore<M> = quest_core::QuestCore<M>;
// pub type MeditationCircle<M> = meditation_circle::MeditationCircle<M>;

mod hero_core {
    use ethers_contract::abigen;
    abigen!(HeroCore, "artifacts/HeroCore.sol/HeroCore.json");

    impl Copy for HeroStats {}
    // impl Clone for HeroStats {}
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
