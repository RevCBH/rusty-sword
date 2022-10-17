// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

interface QuestCoreV2_2 {
    function getAccountActiveQuests(address _account)
        external
        view
        returns (QuestInfo[] memory);

    function completeQuest(uint256 _heroId) external;

    function multiCompleteQuest(uint256[] calldata _heroIds) external;

    function getHeroQuest(uint256 heroId)
        external
        view
        returns (QuestInfo memory);

    function getCurrentStamina(uint256 _heroId) external view returns (uint256);

    function multiStartQuest(
        address[] calldata _questAddress,
        uint256[][] calldata _heroIds,
        uint8[] calldata _attempts,
        uint8[] calldata _level
    ) external;
}

struct QuestInfo {
    uint256 id;
    address questAddress;
    uint8 level;
    uint256[] heroes;
    address player;
    uint256 startBlock;
    uint256 startAtTime;
    uint256 completeAtTime;
    uint8 attempts;
    uint8 status;
}
