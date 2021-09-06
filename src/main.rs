#![allow(non_snake_case)]
#![warn(clippy::too_many_arguments)]
use serde::{Deserialize, Serialize};
use std::net::{IpAddr, SocketAddr};
use warp::Filter;
use rand::distributions::Uniform;
use rand::rngs::ThreadRng;
use rand::thread_rng;
use rand::prelude::IteratorRandom;
use rand::Rng;

const FILE_DATA_GLOBAL: &str = include_str!("pokemons.json");

#[derive(Deserialize, Clone)]
struct Move {
    id: usize,
    game: String,
}

#[derive(Deserialize, Clone)]
struct Ability {
    id: usize,
    gen: String,
}

#[derive(Deserialize, Clone)]
struct Pokemon {
    pokemon_id: usize,
    normal_moves: Vec<Move>,
    egg_moves: Vec<Move>,
    normal_abilities: Vec<Ability>,
    hidden_abilities: Vec<Ability>,
    pokemon_gen: String,
}

#[derive(Serialize)]
struct PokemonStats {
    Species: usize,
    Ability: usize,
    Gender: u8,
    isShiny: bool,
    Nature: u8,
    Hp: u8,
    Atk: u8,
    Def: u8,
    SpA: u8,
    SpD: u8,
    Spe: u8,
    moveOne: usize,
    moveTwo: usize,
    moveThree: usize,
    moveFour: usize,
}

fn convert_gen(gen: &str) -> u8 {
    match gen {
        "generation-i" => 1,
        "generation-ii" => 2,
        "generation-iii" => 3,
        "generation-iv" => 4,
        "generation-v" => 5,
        "generation-vi" => 6,
        "generation-vii" => 7,
        "generation-viii" => 8,
        _ => 3,
    }
}

fn is_gen_lower_or_equal(gen1: &str, gen2: &str) -> bool {
    convert_gen(gen1) <= convert_gen(gen2)
}

fn gen_rand_ability(pokemon: &Pokemon, generation: &str, hidden_ability_chance: usize, rng: &mut ThreadRng) -> usize {
    if !is_gen_lower_or_equal(generation, "generation-ii") {
        let a = pokemon
            .hidden_abilities
            .iter()
            .filter(|x| is_gen_lower_or_equal(&x.gen, generation));
        return if rng.gen_range(0, 101) < hidden_ability_chance
            && !is_gen_lower_or_equal(generation, "generation-iv")
            && a.clone().peekable().peek().is_some()
        {
            a.choose(rng).unwrap().id
        } else {
            pokemon
                .normal_abilities
                .iter()
                .filter(|x| is_gen_lower_or_equal(&x.gen, generation))
                .choose(rng)
                .unwrap()
                .id
        };
    }
    0
}

fn gen_rand_moves(pokemon: &Pokemon, game: &str, egg_move_chance: usize, rng: &mut ThreadRng) -> Vec<usize> {
    let normal_moves = pokemon
        .normal_moves
        .iter()
        .filter_map(|x| match x.game.eq(game) {
            true => Some(x.id),
            false => None,
        });
    let egg_moves = pokemon
        .egg_moves
        .iter()
        .filter_map(|x| match x.game.eq(game) {
            true => Some(x.id),
            false => None,
        });
    if egg_moves.clone().peekable().peek().is_some() {
        let b: usize = (0..3).filter(|_| rng.gen_range(0, 101) < egg_move_chance).count();
        if b != 0 {
            let mut rtrnval = egg_moves.choose_multiple(rng, b);
            rtrnval.append(&mut normal_moves.choose_multiple(rng, 4 - b));
            return rtrnval;
        }
    }
    normal_moves.choose_multiple(rng, 4)
}

fn gen_rand_gender(species: &usize, rng: &mut ThreadRng) -> u8 {
    let genderless_pokemon: [usize; 23] = [
        883, 881, 343, 374, 436, 703, 615, 781, 882, 880, 870, 622, 599, 337, 81, 774, 855, 137,
        479, 338, 120, 201, 100,
    ];

    let female_only_pokemon: [usize; 12] = [29, 314, 440, 115, 238, 241, 548, 629, 669, 761, 856, 868];

    let male_only_pokemon: [usize; 7] = [32, 236, 128, 538, 539, 627, 859];

    if genderless_pokemon.contains(species) {
        2
    } else if female_only_pokemon.contains(species) {
        1
    } else if male_only_pokemon.contains(species) {
        0
    } else {
        rng.gen_range(0, 2)
    }
}

fn new_pokemon(
    file_data: &[Pokemon],
    game: &str,
    egg_move_chance: usize,
    hidden_ability_chance: usize,
    shiny_chance: usize,
    max_ivs: bool,
    rng: &mut ThreadRng,
) -> PokemonStats {
    let generation: &str = match game {
        "red-blue" => "generation-i",
        "yellow" => "generation-i",
        "gold-silver" => "generation-ii",
        "crystal" => "generation-ii",
        "firered-leafgreen" => "generation-iii",
        "ruby-sapphire" => "generation-iii",
        "emerald" => "generation-iii",
        "diamond-pearl" => "generation-iv",
        "platinum" => "generation-iv",
        "heartgold-soulsilver" => "generation-iv",
        "black-white" => "generation-v",
        "black-2-white-2" => "generation-v",
        "x-y" => "generation-vi",
        "omega-ruby-alpha-sapphire" => "generation-vi",
        "sun-moon" => "generation-vii",
        "ultra-sun-ultra-moon" => "generation-vii",
        "sword-shield" => "generation-viii",
        _ => "generation-iii",
    };
    let pokemon: &Pokemon = file_data
        .iter()
        .filter(|x| is_gen_lower_or_equal(&x.pokemon_gen, generation))
        .choose(rng)
        .unwrap();
    let rand_moves = gen_rand_moves(pokemon, game, egg_move_chance, rng);
    let range = Uniform::new(1, 32);
    PokemonStats {
        Species: pokemon.pokemon_id,
        Ability: gen_rand_ability(pokemon, generation, hidden_ability_chance, rng),
        Gender: gen_rand_gender(&pokemon.pokemon_id, rng),
        isShiny: shiny_chance > rng.gen_range(0, 101),
        Nature: rng.gen_range(1, 26),
        Hp: match max_ivs {
            true => 31,
            false => rng.sample(range),
        },
        Atk: match max_ivs {
            true => 31,
            false => rng.sample(range),
        },
        Def: match max_ivs {
            true => 31,
            false => rng.sample(range),
        },
        SpA: match max_ivs {
            true => 31,
            false => rng.sample(range),
        },
        SpD: match max_ivs {
            true => 31,
            false => rng.sample(range),
        },
        Spe: match max_ivs {
            true => 31,
            false => rng.sample(range),
        },
        moveOne: match rand_moves.get(0) {
            None => 0,
            Some(x) => *x,
        },
        moveTwo: match rand_moves.get(1) {
            None => 0,
            Some(x) => *x,
        },
        moveThree: match rand_moves.get(2) {
            None => 0,
            Some(x) => *x,
        },
        moveFour: match rand_moves.get(3) {
            None => 0,
            Some(x) => *x,
        },
    }
}

fn gen_pokemons(
    file_data: &[Pokemon],
    numb_to_gen: usize,
    game: String,
    egg_move_chance: usize,
    hidden_ability_chance: usize,
    shiny_chance: usize,
    maxivs: bool,
    rng: &mut ThreadRng,
) -> String {
    if numb_to_gen > 1000000 {
        return "requested too many eggs to be generated".to_string();
    }
    let mut rtrnval: String = String::from("[");
    for _ in 0..numb_to_gen {
        rtrnval.push_str(&serde_json::to_string::<PokemonStats>(&new_pokemon(
            &file_data,
            &game,
            egg_move_chance,
            hidden_ability_chance,
            shiny_chance,
            maxivs,
            rng,
        )).unwrap());
        rtrnval.push_str(",")
    }
    rtrnval.pop();
    rtrnval.push_str("]");
    rtrnval
}

#[tokio::main]
async fn main() {
    let file_data: Vec<Pokemon> = serde_json::from_str(&FILE_DATA_GLOBAL).unwrap();
    let file_data_clone: Vec<Pokemon> = file_data.clone();
    let maxivs_route = warp::path!("maxivs" / usize / String / usize / usize / usize).map(
        move |numb_to_gen, game, egg_move_chance, hidden_ability_chance, shiny_chance| {
            gen_pokemons(
                &file_data,
                numb_to_gen,
                game,
                egg_move_chance,
                hidden_ability_chance,
                shiny_chance,
                true,
                &mut thread_rng()
            )
        },
    );
    let normal_route = warp::path!(usize / String / usize / usize / usize).map(
        move |numb_to_gen, game, egg_move_chance, hidden_ability_chance, shiny_chance| {
            gen_pokemons(
                &file_data_clone,
                numb_to_gen,
                game,
                egg_move_chance,
                hidden_ability_chance,
                shiny_chance,
                false,
                &mut thread_rng()
            )
        },
    );
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .unwrap();
    let socket = SocketAddr::new(IpAddr::from([0, 0, 0, 0]), port);
    let routes = warp::get().and(maxivs_route.or(normal_route));
    warp::serve(routes).run(socket).await;
}
