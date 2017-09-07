use evaluation::Level;
use types::PokeType;

mod generated_species;
pub use self::generated_species::*;

pub const NUM_SPECIES: u16 = 251;

#[derive(Debug, Eq, PartialEq, Hash)]
/// Information about a specific Pokemon species.
pub struct PokeSpecies {
    id: u16,
    attack: u16,
    defense: u16,
    stamina: u16,
    primary_type: PokeType,
    secondary_type: Option<PokeType>,
    english: &'static str,
    japanese: &'static str,
    japanese_transliterated: &'static str,
    korean: &'static str,
    chinese: &'static str,
    french: &'static str,
    german: &'static str,
    spanish: &'static str,
    italian: &'static str,
}

impl PokeSpecies {
    /// Given the id of a species, return its corresponding data. Return `None` if no data is found
    /// for the given id.
    ///
    /// ## Example:
    ///
    /// ```
    /// use pokemon_go_data::species::PokeSpecies;
    ///
    /// let dragonite = PokeSpecies::from_id(149).unwrap();
    /// assert_eq!((263, 201, 182), dragonite.base_stats());
    ///
    /// let no_such_pokemon = PokeSpecies::from_id(500);
    /// assert!(no_such_pokemon.is_none());
    /// ```
    pub fn from_id(id: u16) -> Option<&'static PokeSpecies> {
        if id == 0 || id > NUM_SPECIES - 1 {
            return None
        }

        Some(generated_species::ALL_SPECIES[(id - 1) as usize])
    }

    /// Return a list of all `PokeSpecies`, sorted by id.
    ///
    /// ## Example:
    ///
    /// ```
    /// use pokemon_go_data::species::PokeSpecies;
    ///
    /// let all_species = PokeSpecies::all_species();
    ///
    /// assert_eq!(251, all_species.len());
    /// assert_eq!(1, all_species[0].id());
    /// ```
    pub fn all_species() -> &'static [&'static PokeSpecies] {
        generated_species::ALL_SPECIES
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

    pub fn english(&self) -> &'static str {
        self.english
    }

    pub fn japanese(&self) -> &'static str {
        self.japanese
    }

    pub fn japanese_transliterated(&self) -> &'static str {
        self.japanese_transliterated
    }

    pub fn korean(&self) -> &'static str {
        self.korean
    }

    pub fn chinese(&self) -> &'static str {
        self.chinese
    }

    pub fn french(&self) -> &'static str {
        self.french
    }

    pub fn german(&self) -> &'static str {
        self.german
    }

    pub fn spanish(&self) -> &'static str {
        self.spanish
    }

    pub fn italian(&self) -> &'static str {
        self.italian
    }

    /// Returns the highest possible stats for this pokemon (e.g., perfect IVs).
    pub fn perfect_stats(&self) -> (u16, u16, u16) {
        (self.attack() + 15, self.defense() + 15, self.stamina() + 15)
    }

    /// Returns the species' max combat power at the given `Level`, assuming perfect IVs.
    ///
    /// ## Example:
    ///
    /// ```
    /// use pokemon_go_data::{Level, species};
    ///
    /// let level_30 = Level::new(30).unwrap();
    /// assert_eq!(188, species::Magikarp.max_cp_at_level(level_30));
    /// ```
   pub fn max_cp_at_level(&self, level: Level) -> u16 {
        let (attack, defense, stamina) = self.perfect_stats();

        PokeSpecies::calculate_cp(attack, defense, stamina, level)
    }

    /// Returns this species' max Combat Power at level 39, assuming perfect IVs.
    ///
    /// ## Example:
    ///
    /// ```
    /// use pokemon_go_data::species;
    ///
    /// assert_eq!(3_617, species::Tyranitar.max_cp());
    /// ```
    pub fn max_cp(&self) -> u16 {
        let (attack, defense, stamina) = self.perfect_stats();

        PokeSpecies::calculate_cp(attack, defense, stamina, Level::max())
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
    use species::*;

    #[test]
    fn species_by_id() {
        let pokemon = PokeSpecies::from_id(1).unwrap();
        assert_eq!(Bulbasaur, pokemon);

        let pokemon = PokeSpecies::from_id(161).unwrap();
        assert_eq!(Sentret, pokemon);

        let pokemon = PokeSpecies::from_id(249).unwrap();
        assert_eq!(Lugia, pokemon);

        assert!(PokeSpecies::from_id(252).is_none());
    }

    #[test]
    fn poketype() {
        assert_eq!(PokeType::Fire, Charizard.primary_type());
        assert_eq!(Some(PokeType::Flying), Charizard.secondary_type());

        assert_eq!(PokeType::Water, Squirtle.primary_type());
        assert!(Squirtle.secondary_type().is_none());
    }

    #[test]
    fn all_species() {
        let all_pokemon = PokeSpecies::all_species();
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

        assert_eq!(224, Kakuna.max_cp_at_level(level));
    }

    #[test]
    fn max_cp() {
        assert_eq!(955, Eevee.max_cp());
        assert_eq!(3_530, Dragonite.max_cp());
    }
}
