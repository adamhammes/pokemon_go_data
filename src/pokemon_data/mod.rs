use evaluation::Level;
use types::PokeType;

mod generated_species;
use self::generated_species::SPECIES;

#[derive(Debug, Eq, PartialEq, Hash)]
/// Information about a specific Pokemon species.
pub struct Species {
    id: u16,
    attack: u16,
    defense: u16,
    stamina: u16,
    primary_type: PokeType,
    secondary_type: Option<PokeType>,
}

impl Species {
    /// Given the id of a species, return its corresponding data. Return `None` if no data is found
    /// for the given id.
    ///
    /// ## Example:
    ///
    /// ```
    /// use pokemon_go_data::Species;
    ///
    /// let dragonite = Species::from_id(149).unwrap();
    /// assert_eq!((263, 201, 182), dragonite.base_stats());
    ///
    /// let no_such_pokemon = Species::from_id(500);
    /// assert!(no_such_pokemon.is_none());
    /// ```
    pub fn from_id(id: u16) -> Option<&'static Species> {
        SPECIES.iter().find(|p| p.id() == id)
    }

    /// Return a list of all `Species`, sorted by id.
    ///
    /// ## Example:
    ///
    /// ```
    /// use pokemon_go_data::Species;
    ///
    /// let all_species = Species::all_species();
    ///
    /// assert_eq!(251, all_species.len());
    /// assert_eq!(1, all_species[0].id());
    /// ```
    pub fn all_species() -> &'static [Species] {
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

    /// Returns the highest possible stats for this pokemon (e.g., perfect IVs).
    pub fn perfect_stats(&self) -> (u16, u16, u16) {
        (self.attack() + 15, self.defense() + 15, self.stamina() + 15)
    }

    /// Returns the species' max Combat Power at the given `Level`, assuming perfect IVs.
    pub fn max_cp_at_level(&self, level: Level) -> u16 {
        let (attack, defense, stamina) = self.perfect_stats();

        Species::calculate_cp(attack, defense, stamina, level)
    }

    /// Returns this species' max Combat Power at level 39, assuming perfect IVs.
    pub fn max_cp(&self) -> u16 {
        let (attack, defense, stamina) = self.perfect_stats();

        Species::calculate_cp(attack, defense, stamina, Level::max())
    }

    fn calculate_cp(attack: u16, defense: u16, stamina: u16, level: Level) -> u16 {
        let attack_calc = attack as f64;
        let defense_calc = (defense as f64).sqrt();
        let stamina_calc = (stamina as f64).sqrt();

        let cp_multiplier = level.cp_multiplier().powi(2);

        let inner = attack_calc * defense_calc * stamina_calc * cp_multiplier / 10.;

        ::std::cmp::max(10, inner.floor() as u16)
    }
}

#[cfg(test)]
mod tests {
    use evaluation::Level;
    use types::PokeType;
    use Species;

    #[test]
    fn species_by_id() {
        let bulbasaur = Species::from_id(1).unwrap();
        assert_eq!((118, 118, 90), bulbasaur.base_stats());

        let sentret = Species::from_id(161).unwrap();
        assert_eq!((79, 77, 70), sentret.base_stats());

        let lugia = Species::from_id(249).unwrap();
        assert_eq!((193, 323, 212), lugia.base_stats());

        assert!(Species::from_id(252).is_none());
    }

    #[test]
    fn poketype() {
        let charizard = Species::from_id(6).unwrap();
        assert_eq!(PokeType::Fire, charizard.primary_type());
        assert_eq!(Some(PokeType::Flying), charizard.secondary_type());

        let squirtle = Species::from_id(7).unwrap();
        assert_eq!(PokeType::Water, squirtle.primary_type());
        assert!(squirtle.secondary_type().is_none());
    }

    #[test]
    fn all_species() {
        let all_pokemon = Species::all_species();
        let num_pokemon = 251;

        assert_eq!(num_pokemon, all_pokemon.len());

        for i in 0..num_pokemon {
            assert!(
                all_pokemon[i].id() == (i + 1) as u16,
                format!("Couldn't find pokemon #{}", i)
            );
        }
    }

    #[test]
    fn max_cp_at_level() {
        let level = Level::new(20).unwrap();
        let kakuna = Species::from_id(14).unwrap();

        assert_eq!(224, kakuna.max_cp_at_level(level));
    }

    #[test]
    fn max_cp() {
        let eevee = Species::from_id(133).unwrap();
        assert_eq!(955, eevee.max_cp());

        let dragonite = Species::from_id(149).unwrap();
        assert_eq!(3_530, dragonite.max_cp());
    }
}
