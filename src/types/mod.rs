use std::str::FromStr;

mod generated;
use self::generated::EFFECTIVENESS_CHART;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Effectiveness {
    DoubleNotVery,
    NotVery,
    Normal,
    SuperEffective
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum PokeType {
    Normal = 0,
    Fighting = 1,
    Flying = 2,
    Poison = 3,
    Ground = 4,
    Rock = 5,
    Bug = 6,
    Ghost = 7,
    Steel = 8,
    Fire = 9,
    Water = 10,
    Grass = 11,
    Electric = 12,
    Psychic = 13,
    Ice = 14,
    Dragon = 15,
    Dark = 16,
    Fairy = 17
}

impl PokeType {
    pub fn efficacy_against(&self, other: PokeType) -> Effectiveness {
        EFFECTIVENESS_CHART[*self as usize][other as usize]
    }
}

impl FromStr for PokeType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lowercase = s.to_lowercase();

        match lowercase.as_ref() {
            "normal" => Ok(PokeType::Normal),
            "fighting" => Ok(PokeType::Fighting),
            "flying" => Ok(PokeType::Flying),
            "poison" => Ok(PokeType::Poison),
            "ground" => Ok(PokeType::Ground),
            "rock" => Ok(PokeType::Rock),
            "bug" => Ok(PokeType::Bug),
            "ghost" => Ok(PokeType::Ghost),
            "steel" => Ok(PokeType::Steel),
            "fire" => Ok(PokeType::Fire),
            "water" => Ok(PokeType::Water),
            "grass" => Ok(PokeType::Grass),
            "electric" => Ok(PokeType::Electric),
            "psychic" => Ok(PokeType::Psychic),
            "ice" => Ok(PokeType::Ice),
            "dragon" => Ok(PokeType::Dragon),
            "dark" => Ok(PokeType::Dark),
            "fairy" => Ok(PokeType::Fairy),
            _ => Err(())
        }
    }
}

#[cfg(test)]
mod tests {
    use ::{PokeType, Effectiveness};

    #[test]
    fn efficacy_against() {
        let electric_v_ground = PokeType::Electric.efficacy_against(PokeType::Ground);
        assert_eq!(Effectiveness::DoubleNotVery, electric_v_ground);

        let grass_v_bug = PokeType::Grass.efficacy_against(PokeType::Bug);
        assert_eq!(Effectiveness::NotVery, grass_v_bug);

        let poison_v_flying = PokeType::Poison.efficacy_against(PokeType::Flying);
        assert_eq!(Effectiveness::Normal, poison_v_flying);

        let fighting_v_normal = PokeType::Fighting.efficacy_against(PokeType::Normal);
        assert_eq!(Effectiveness::SuperEffective, fighting_v_normal);
    }
}
