use types::PokeType;

mod generated_species_data;
use self::generated_species_data::SPECIES;

#[derive(Debug, Eq, PartialEq, Hash)]
/// Information about a specific Pokemon species.
pub struct SpeciesData {
    id: u16,
    attack: u16,
    defense: u16,
    stamina: u16,
    primary_type: PokeType,
    secondary_type: Option<PokeType>
}


impl SpeciesData {
    /// Given the id of a species, return its corresponding data. Return `None` if no data is found for
    /// the given id.
    ///
    /// ## Example:
    ///
    /// ```
    /// use pokemon_go_data::SpeciesData;
    ///
    /// let dragonite = SpeciesData::from_id(149).unwrap();
    /// assert_eq!((263, 201, 182), dragonite.base_stats());
    ///
    /// let no_such_pokemon = SpeciesData::from_id(500);
    /// assert!(no_such_pokemon.is_none());
    /// ```
    pub fn from_id(id: u16) -> Option<&'static SpeciesData> {
        SPECIES
            .iter()
            .find(|p| p.id() == id)
    }

    /// Return a list of all `SpeciesData`, sorted by id.
    ///
    /// ## Example:
    ///
    /// ```
    /// use pokemon_go_data::SpeciesData;
    /// let all_species = SpeciesData::all_species();
    ///
    /// assert_eq!(251, all_species.len());
    /// assert_eq!(1, all_species[0].id());
    /// ```
    pub fn all_species() -> &'static [SpeciesData] {
        SPECIES
    }

    /// This species' id.
    pub fn id(&self) -> u16 {
        self.id
    }

    /// This species' base attack value.
    pub fn attack(&self) -> u16 {
        self.attack
    }

    /// This species's base defense value.
    pub fn defense(&self) -> u16 {
        self.defense
    }

    /// This species' base stamina value.
    pub fn stamina(&self) -> u16 {
        self.stamina
    }

    /// This species' base stats, as an `(attack, defense, stamina)` triple.
    pub fn base_stats(&self) -> (u16, u16, u16) {
        (self.attack, self.defense, self.stamina)
    }

    /// This species' primary `PokeType`.
    pub fn primary_type(&self) -> PokeType {
        self.primary_type
    }

    /// If this species is dual-typed, returns the secondary `PokeType`.
    pub fn secondary_type(&self) -> Option<PokeType> {
        self.secondary_type
    }

    /// Returns `true` if this species has two `PokeType`s.
    pub fn is_dual_type(&self) -> bool {
        self.secondary_type.is_some()
    }
}

#[cfg(test)]
mod tests {
    use types::PokeType;
    use ::SpeciesData;

    #[test]
    fn pokemon_by_id_test() {
        let bulbasaur = SpeciesData::from_id(1).unwrap();
        assert_eq!((118, 118, 90), bulbasaur.base_stats());

        let sentret = SpeciesData::from_id(161).unwrap();
        assert_eq!((79, 77, 70), sentret.base_stats());

        let lugia = SpeciesData::from_id(249).unwrap();
        assert_eq!((193, 323, 212), lugia.base_stats());

        assert!(SpeciesData::from_id(252).is_none());
    }

    #[test]
    fn poketype() {
        let charizard = SpeciesData::from_id(6).unwrap();
        assert_eq!(PokeType::Fire, charizard.primary_type());
        assert_eq!(Some(PokeType::Flying), charizard.secondary_type());

        let squirtle = SpeciesData::from_id(7).unwrap();
        assert_eq!(PokeType::Water, squirtle.primary_type());
        assert!(squirtle.secondary_type().is_none());
    }

    #[test]
    fn all_pokemon_test() {
        let all_pokemon = SpeciesData::all_species();
        let num_pokemon = 251;

        assert_eq!(num_pokemon, all_pokemon.len());

        for i in 0..num_pokemon {
            assert!(all_pokemon[i].id() == (i + 1) as u16, format!("Couldn't find pokemon #{}", i));
        }
    }
}