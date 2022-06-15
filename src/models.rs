use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone)]
pub struct Move {
    pub id: usize,
    pub game: String,
}

#[derive(Deserialize, Clone)]
pub struct Ability {
    pub id: usize,
    pub gen: String,
}

#[derive(Deserialize, Clone)]
pub struct Pokemon {
    pub pokemon_id: usize,
    pub normal_moves: Vec<Move>,
    pub egg_moves: Vec<Move>,
    pub normal_abilities: Vec<Ability>,
    pub hidden_abilities: Vec<Ability>,
    pub pokemon_gen: String,
}

#[derive(Serialize)]
pub struct PokemonStats {
    pub Species: usize,
    pub Ability: usize,
    pub Gender: u8,
    pub isShiny: bool,
    pub Nature: u8,
    pub Hp: u8,
    pub Atk: u8,
    pub Def: u8,
    pub SpA: u8,
    pub SpD: u8,
    pub Spe: u8,
    pub moveOne: usize,
    pub moveTwo: usize,
    pub moveThree: usize,
    pub moveFour: usize,
}


