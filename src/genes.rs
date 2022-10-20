use ethers_core::types::U256;
use eyre::{eyre, Result};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(Clone, Copy, Debug)]
pub enum Gender {
    Female,
    Male,
}

#[derive(Clone, Copy, Debug)]
pub struct VisualTraits {
    pub gender: Gender,
    // HeadAppendage,
    // BackAppendage,
    // Background,
    // HairStyle,
    // HairColor,
    // VisualUnknown1,
    // EyeColor,
    // SkinColor,
    // AppendageColor,
    // BackAppendageColor,
    // VisualUnknown2,
}

#[derive(Clone, Copy, Debug)]
pub struct StatTraits {
    pub class: Class,
    pub subclass: Class,
    pub profession: Profession,
    //pub passive1: Passive1,
    //pub passive2: Passive2,
    //pub active1: Active1,
    //pub active2: Active2,
    //pub statBoost1: StatBoost1,
    //pub statBoost2: StatBoost2,
    //pub statsUnknown1: StatsUnknown1,
    //pub element: Element,
    //pub statsUnknown2: StatsUnknown2,
}

#[derive(Clone, Copy, FromPrimitive, PartialEq, Debug)]
pub enum Class {
    Warrior = 0,
    Knight = 1,
    Thief = 2,
    Archer = 3,
    Priest = 4,
    Wizard = 5,
    Monk = 6,
    Pirate = 7,
    Paladin = 16,
    DarkKnight = 17,
    Summoner = 18,
    Ninja = 19,
    Dragoon = 24,
    Sage = 25,
    DreadKnight = 28,
}

#[derive(Clone, Copy, FromPrimitive, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Profession {
    Mining = 0,
    Gardening = 2,
    Fishing = 4,
    Foraging = 6,
}

#[derive(Clone, Copy, FromPrimitive, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum BaseStat {
    Strength,
    Dexterity,
    Agility,
    Vitality,
    Endurance,
    Intelligence,
    Wisdom,
    Luck,
}

const BASE: usize = 32;
fn parse_genes(genes: &U256) -> Vec<u8> {
    let mut genes = genes.clone();
    let mut result: Vec<u8> = vec![];
    while genes.ge(&BASE.into()) {
        let (rest, gene) = genes.div_mod(BASE.into());
        genes = rest;
        result.push(gene.as_u32() as u8);
    }
    result.push(genes.as_u32() as u8);

    while result.len() < 48 as usize {
        result.push(0.into());
    }
    result.reverse();

    result
}

pub fn parse_stat_genes(genes: &U256) -> Result<StatTraits> {
    let genes = parse_genes(genes);

    let gene = genes.get(3).ok_or(eyre!("bad index for class gene"))?;
    let class_trait: Class = Class::from_u8(*gene).ok_or(eyre!("invalid class gene: {}", gene))?;

    let gene = genes.get(7).ok_or(eyre!("bad index for subclass gene"))?;
    let subclass_trait: Class =
        Class::from_u8(*gene).ok_or(eyre!("invalid subclass gene: {}", gene))?;

    let gene = genes
        .get(11)
        .ok_or(eyre!("bad index for profession gene"))?;
    let profession_trait: Profession =
        Profession::from_u8(*gene).ok_or(eyre!("invalid profession gene"))?;

    Ok(StatTraits {
        class: class_trait,
        subclass: subclass_trait,
        profession: profession_trait,
    })
}

pub fn parse_visual_genes(genes: &U256) -> Result<VisualTraits> {
    let genes = parse_genes(genes);
    let gender = match genes.get(0).ok_or(eyre!("bad index for gender gene"))? {
        1 => Gender::Male,
        _ => Gender::Female,
    };

    Ok(VisualTraits { gender: gender })
}

#[cfg(test)]
mod tests {
    use ethers_core::types::U256;

    use super::{parse_genes, parse_stat_genes};
    use super::{Class::*, Profession::*};

    #[test]
    fn test_parse_genes() {
        let genes: U256 = U256::from_dec_str(
            "62493423265980524509052052826089036949624339886608478857262822890545358",
        )
        .expect("bad genes decimal number string");

        assert_eq!(
            parse_genes(&genes),
            vec![
                1, 4, 7, 0, 5, 2, 6, 7, 0, 4, 2, 6, 5, 4, 5, 0, 4, 6, 3, 7, 0, 6, 2, 2, 6, 0, 3, 6,
                6, 0, 8, 4, 12, 10, 8, 2, 1, 0, 4, 4, 8, 10, 6, 4, 4, 8, 6, 14
            ]
        );
    }

    #[test]
    fn test_parse_stat_genes() {
        let genes: U256 = U256::from_dec_str(
            "62493423265980524509052052826089036949624339886608478857262822890545358",
        )
        .expect("bad genes decimal number string");

        let traits = parse_stat_genes(&genes).expect("failed to parse genes");

        assert_eq!(
            traits.class, Warrior,
            "Expected class Warrior, got {:?}",
            traits.class
        );

        assert_eq!(
            traits.subclass, Pirate,
            "Expected subclass Pirate, got {:?}",
            traits.subclass
        );

        assert_eq!(
            traits.profession, Foraging,
            "Expected profession Foraging, got {:?}",
            traits.profession
        );
    }
}
