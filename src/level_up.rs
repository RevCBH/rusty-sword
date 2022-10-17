// use crate::util::hashmap;

pub fn xp_to_level(n: u16) -> Option<u64> {
    match n {
        1 => Some(2000),
        2 => Some(3000),
        3 => Some(4000),
        4 => Some(5000),
        5 => Some(6000),
        6 => Some(8000),
        7 => Some(10000),
        8 => Some(12000),
        9 => Some(16000),
        10 => Some(20000),
        11 => Some(24000),
        12 => Some(28000),
        13 => Some(32000),
        14 => Some(36000),
        15 => Some(40000),
        16 => Some(45000),
        17 => Some(50000),
        18 => Some(55000),
        19 => Some(60000),
        20 => Some(65000),
        _ => None,
    }
}
