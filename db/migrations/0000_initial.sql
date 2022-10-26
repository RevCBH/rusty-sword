CREATE TABLE meditation_plans (
    hero_id TEXT NOT NULL,
    from_level INTEGER NOT NULL,
    primary_attribute INTEGER NOT NULL,
    secondary_attribute1 INTEGER NOT NULL,
    secondary_attribute2 INTEGER NOT NULL,
    boost_item TEXT,
    PRIMARY KEY(hero_id, from_level),
    FOREIGN KEY(hero_id) REFERENCES hero_info(id)
);

CREATE TABLE heros (
    id TEXT PRIMARY KEY,
    visual_genes TEXT NOT NULL,
    stat_genes TEXT NOT NULL,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    "level" INTEGER NOT NULL,
    xp TEXT NOT NULL, -- wants to be u64
    strength INTEGER NOT NULL,
    dexterity INTEGER NOT NULL,
    agility INTEGER NOT NULL,
    vitality INTEGER NOT NULL,
    endurance INTEGER NOT NULL,
    intelligence INTEGER NOT NULL,
    wisdom INTEGER NOT NULL,
    luck INTEGER NOT NULL
)