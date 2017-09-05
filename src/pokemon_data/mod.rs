use types::PokeType;

mod generated;
use self::generated::POKE_STATS;

#[derive(Debug, Eq, PartialEq, Hash)]
/// The base stats of a Pokemon.
pub struct PokemonData {
    id: u16,
    attack: u16,
    defense: u16,
    stamina: u16,
    primary_type: PokeType,
    secondary_type: Option<PokeType>
}

impl PokemonData {
    /// The Pokemon's id.
    pub fn id(&self) -> u16 {
        self.id
    }

    /// The Pokemon's base attack value.
    pub fn attack(&self) -> u16 {
        self.attack
    }

    /// The Pokemon's base defense value.
    pub fn defense(&self) -> u16 {
        self.defense
    }

    /// The Pokemon's base stamina value.
    pub fn stamina(&self) -> u16 {
        self.stamina
    }

    /// The Pokemon's base stats, as a `(attack, defense, stamina)` triple.
    pub fn base_stats(&self) -> (u16, u16, u16) {
        (self.attack, self.defense, self.stamina)
    }

    pub fn primary_type(&self) -> PokeType {
        self.primary_type
    }

    pub fn secondary_type(&self) -> Option<PokeType> {
        self.secondary_type
    }
}

/// Given the id of a Pokemon, return its corresponding data. Return `None` if no data is found for
/// the given id.
///
/// ## Example:
///
/// ```
/// use pokemon_go_data::pokemon_by_id;
///
/// let dragonite = pokemon_by_id(149).unwrap();
/// assert_eq!((263, 201, 182), dragonite.base_stats());
///
/// let no_such_pokemon = pokemon_by_id(500);
/// assert!(no_such_pokemon.is_none());
/// ```
pub fn pokemon_by_id(id: u16) -> Option<&'static PokemonData> {
    POKE_STATS
        .iter()
        .find(|p| p.id() == id)
}

/// Return a list of all `PokeData`, sorted by id.
///
/// ## Example:
///
/// ```
/// use pokemon_go_data::all_pokemon;
/// let all_pokemon = all_pokemon();
///
/// assert_eq!(251, all_pokemon.len());
/// assert_eq!(1, all_pokemon[0].id());
/// ```
pub fn all_pokemon() -> &'static [PokemonData] {
    POKE_STATS
}

#[cfg(test)]
mod tests {
    use types::PokeType;
    use ::{all_pokemon, pokemon_by_id};

    #[test]
    fn pokemon_by_id_test() {
        let bulbasaur = pokemon_by_id(1).unwrap();
        assert_eq!((118, 118, 90), bulbasaur.base_stats());

        let sentret = pokemon_by_id(161).unwrap();
        assert_eq!((79, 77, 70), sentret.base_stats());

        let lugia = pokemon_by_id(249).unwrap();
        assert_eq!((193, 323, 212), lugia.base_stats());

        assert!(pokemon_by_id(444).is_none());
    }

    #[test]
    fn poketype() {
        let charizard = pokemon_by_id(6).unwrap();
        assert_eq!(PokeType::Fire, charizard.primary_type());
        assert_eq!(Some(PokeType::Flying), charizard.secondary_type());

        let squirtle = pokemon_by_id(7).unwrap();
        assert_eq!(PokeType::Water, squirtle.primary_type());
        assert!(squirtle.secondary_type().is_none());
    }

    #[test]
    fn all_pokemon_test() {
        let all_pokemon = all_pokemon();
        let num_pokemon = 251;

        assert_eq!(num_pokemon, all_pokemon.len());

        for i in 0..num_pokemon {
            assert!(all_pokemon[i].id() == (i + 1) as u16, format!("Couldn't find pokemon #{}", i));
        }
    }
}