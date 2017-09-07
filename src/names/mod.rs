mod generated_names;
pub use self::generated_names::*;

pub struct PokeName {
    id: u16,
    english: &'static str,
}

impl PokeName {
    pub fn id(&self) -> u16 {
        self.id
    }

    pub fn english(&self) -> &'static str {
        self.english
    }
}