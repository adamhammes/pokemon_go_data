//! Holds the `PokeName` constants.
mod generated_names;
pub use self::generated_names::*;

/// Provides information about a Pokemon's name. Also used to access specific Pokemon in a type-safe
/// manner (see [Species::named](../struct.Species.html#method.named) for an example).
pub struct PokeName {
    id: u16,
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

impl PokeName {
    pub fn id(&self) -> u16 {
        self.id
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
}
