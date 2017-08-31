extern crate csv;
extern crate serde;

#[macro_use]
extern crate serde_derive;

use std::env;
use std::fs;
use std::io::{Write};
use std::path;

#[derive(Debug, Deserialize)]
struct PokemonData {
    id: u16,
    stamina: u16,
    attack: u16,
    defense: u16
}

fn main() {
    let mut reader = csv::Reader::from_path("build/pokemon_stats.tsv").unwrap();

    let mut stats: Vec<PokemonData> = reader
        .deserialize()
        .map(|r| r.unwrap())
        .collect();

    stats.sort_by_key(|p| p.id);

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = path::Path::new(&out_dir).join("data.rs" );
    let mut f = fs::File::create(&dest_path).unwrap();

    write!(&mut f, "const POKE_STATS: &'static [PokemonData] = &[\n").unwrap();

    for data in stats {
        write!(&mut f, "    PokemonData {{ id: {}, attack: {}, defense: {}, stamina: {} }},\n",
            data.id, data.attack, data.defense, data.stamina).unwrap();
    }

    write!(&mut f,"];\n").unwrap();
}
