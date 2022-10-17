// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

interface MeditationCircle {
    // event AttunementCrystalAdded(address atunementItemAddress);
    // event LevelUp(address indexed player, uint256 indexed heroId, tuple(uint256 id, tuple(uint256 summonedTime, uint256 nextSummonTime, uint256 summonerId, uint256 assistantId, uint32 summons, uint32 maxSummons) summoningInfo, tuple(uint256 statGenes, uint256 visualGenes, uint8 rarity, bool shiny, uint16 generation, uint32 firstName, uint32 lastName, uint8 shinyStyle, uint8 class, uint8 subClass) info, tuple(uint256 staminaFullAt, uint256 hpFullAt, uint256 mpFullAt, uint16 level, uint64 xp, address currentQuest, uint8 sp, uint8 status) state, tuple(uint16 strength, uint16 intelligence, uint16 wisdom, uint16 luck, uint16 agility, uint16 vitality, uint16 endurance, uint16 dexterity, uint16 hp, uint16 mp, uint16 stamina) stats, tuple(uint16 strength, uint16 intelligence, uint16 wisdom, uint16 luck, uint16 agility, uint16 vitality, uint16 endurance, uint16 dexterity, uint16 hpSm, uint16 hpRg, uint16 hpLg, uint16 mpSm, uint16 mpRg, uint16 mpLg) primaryStatGrowth, tuple(uint16 strength, uint16 intelligence, uint16 wisdom, uint16 luck, uint16 agility, uint16 vitality, uint16 endurance, uint16 dexterity, uint16 hpSm, uint16 hpRg, uint16 hpLg, uint16 mpSm, uint16 mpRg, uint16 mpLg) secondaryStatGrowth, tuple(uint16 mining, uint16 gardening, uint16 foraging, uint16 fishing) professions) hero, tuple(uint256 id, tuple(uint256 summonedTime, uint256 nextSummonTime, uint256 summonerId, uint256 assistantId, uint32 summons, uint32 maxSummons) summoningInfo, tuple(uint256 statGenes, uint256 visualGenes, uint8 rarity, bool shiny, uint16 generation, uint32 firstName, uint32 lastName, uint8 shinyStyle, uint8 class, uint8 subClass) info, tuple(uint256 staminaFullAt, uint256 hpFullAt, uint256 mpFullAt, uint16 level, uint64 xp, address currentQuest, uint8 sp, uint8 status) state, tuple(uint16 strength, uint16 intelligence, uint16 wisdom, uint16 luck, uint16 agility, uint16 vitality, uint16 endurance, uint16 dexterity, uint16 hp, uint16 mp, uint16 stamina) stats, tuple(uint16 strength, uint16 intelligence, uint16 wisdom, uint16 luck, uint16 agility, uint16 vitality, uint16 endurance, uint16 dexterity, uint16 hpSm, uint16 hpRg, uint16 hpLg, uint16 mpSm, uint16 mpRg, uint16 mpLg) primaryStatGrowth, tuple(uint16 strength, uint16 intelligence, uint16 wisdom, uint16 luck, uint16 agility, uint16 vitality, uint16 endurance, uint16 dexterity, uint16 hpSm, uint16 hpRg, uint16 hpLg, uint16 mpSm, uint16 mpRg, uint16 mpLg) secondaryStatGrowth, tuple(uint16 mining, uint16 gardening, uint16 foraging, uint16 fishing) professions) oldHero);;
    // event MeditationBegun(address indexed player, uint256 indexed heroId, uint256 meditationId, uint8 primaryStat, uint8 secondaryStat, uint8 tertiaryStat, address attunementCrystal);;
    // event MeditationCompleted(address indexed player, uint256 indexed heroId, uint256 meditationId);    ;
    // event StatUp(address indexed player, uint256 indexed heroId, uint256 stat, uint8 increase, uint8 updateType);;

    function _getRequiredRunes(uint16 _level)
        external
        pure
        returns (uint16[10] calldata);

    function activeAttunementCrystals(address) external view returns (bool);

    function completeMeditation(uint256 _heroId) external;

    // function getActiveMeditations(address _address) view returns (tuple(uint256 id, address player, uint256 heroId, uint8 primaryStat, uint8 secondaryStat, uint8 tertiaryStat, address attunementCrystal, uint256 startBlock, uint8 status)[]);
    // function getHeroMeditation(uint256 _heroId) view returns (tuple(uint256 id, address player, uint256 heroId, uint8 primaryStat, uint8 secondaryStat, uint8 tertiaryStat, address attunementCrystal, uint256 startBlock, uint8 status));
    // function getMeditation(uint256 _id) view returns (tuple(uint256 id, address player, uint256 heroId, uint8 primaryStat, uint8 secondaryStat, uint8 tertiaryStat, address attunementCrystal, uint256 startBlock, uint8 status));
    // function heroToMeditation(uint256) external view returns (uint256);
    // function paused() external view returns (bool);

    function profileActiveMeditations(address, uint256)
        external
        view
        returns (
            uint256 id,
            address player,
            uint256 heroId,
            uint8 primaryStat,
            uint8 secondaryStat,
            uint8 tertiaryStat,
            address attunementCrystal,
            uint256 startBlock,
            uint8 status
        );

    function runes(uint256) external view returns (address);

    function startMeditation(
        uint256 _heroId,
        uint8 _primaryStat,
        uint8 _secondaryStat,
        uint8 _tertiaryStat,
        address _attunementCrystal
    ) external;
}
