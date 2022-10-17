Usage is pretty janky right now:
```bash
$ cargo build
$ HARMONY_PRIVATE_KEY=$(cat /path/to/your/private/key) target/debug/rusty-sword

# To run it every five minutes
$ watch -n 300 HARMONY_PRIVATE_KEY=$(cat /path/to/your/private/key) target/debug/rusty-sword
```

It will:
1. Complete any running quests
2. Look at the state of heros on chain
3. Start quests for any heros with > 15 stamina, in the order of
    1. Fishing/Foraging for all proficient heros
    2. Crystal and Gold mining, if they're not in progress and there are proficient heros
    3. Gardens specified in `config/dfkchain/mainnet.toml`, if they're not in progress and there are proficient heros
    4. Stat quests based on highest stat of ready heros

# Known Issues
* Sometimes, it can't finish a quest and reverts with a "no quest found" error.