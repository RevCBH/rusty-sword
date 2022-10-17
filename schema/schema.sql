CREATE TABLE IF NOT EXISTS meditation_plan (
    hero_id INTEGER NOT NULL,
    from_level INTEGER NOT NULL,
    primary_attribute INTEGER NOT NULL,
    secondary_attribute1 INTEGER NOT NULL,
    secondary_attribute2 INTEGER NOT NULL,
    boost_item TEXT,

    PRIMARY KEY(hero_id, from_level),
    FOREIGN KEY(hero_id) REFERENCES hero_info(id)
);

CREATE TABLE IF NOT EXISTS hero_info (
    id INTEGER PRIMARY KEY,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    "level" INTEGER NOT NULL,
    xp INTEGER NOT NULL
)