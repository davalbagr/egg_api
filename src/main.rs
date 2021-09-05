use warp::Filter;
use serde::{Deserialize, Serialize};
use std::env;
use std::net::SocketAddr;
use std::net::IpAddr;

const FILE_DATA_GLOBAL: &str = include_str!("pokemons.json");

#[derive(Deserialize, Clone)]
struct Move {
    id: usize,
    game: String
}

#[derive(Deserialize, Clone)]
struct Ability {
    id: usize,
    gen: String
}

#[derive(Deserialize, Clone)]
struct Pokemon {
    pokemon_id: usize,
    normal_moves: Vec<Move>,
    egg_moves: Vec<Move>,
    normal_abilities: Vec<Ability>,
    hidden_abilities: Vec<Ability>,
    pokemon_gen: String
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
    moveFour: usize

}

fn game_to_gen(game: &str) -> &'static str {
    match game {
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
        _ => "generation-iii"
    }
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
        _ => 3
    }
}

fn is_gen_lower_or_equal(gen1: &str, gen2: &str) -> bool {
    convert_gen(gen1) <= convert_gen(gen2)
}

fn gen_rand_ability(pokemon: &Pokemon, generation: &str, hidden_ability_chance: usize) -> usize {
    use rand::Rng;
    use rand::prelude::IteratorRandom;
    if !is_gen_lower_or_equal(generation, "generation-ii") {
        let a = pokemon.hidden_abilities.clone().into_iter().filter(|x| { is_gen_lower_or_equal(x.gen.as_str(), generation) });
        let mut rng = rand::thread_rng();
        return if rng.gen_range(0, 100) < hidden_ability_chance && !is_gen_lower_or_equal(generation, "generation-iv") && a.clone().peekable().peek().is_some() {
            a.choose(&mut rng).unwrap().id
        } else {
            pokemon.normal_abilities.iter().filter(|x| { is_gen_lower_or_equal(x.gen.as_str(), generation) }).choose(&mut rng).unwrap().id
        }
    }
    0
}

fn gen_rand_moves(pokemon: &Pokemon, game: &str, egg_move_chance: usize) -> Vec<usize> {
    use rand::Rng;
    use rand::prelude::IteratorRandom;
    let normal_moves: Vec<usize> = pokemon.normal_moves.iter().filter_map(|x| match x.game.eq(game) {
        true => Some(x.id),
        false => None
    }).collect();
    let egg_moves: Vec<usize> = pokemon.egg_moves.iter().filter_map(|x| match x.game.eq(game) {
        true => Some(x.id),
        false => None
    }).collect();
    let mut rng = rand::thread_rng();
    if !egg_moves.is_empty() {
        let mut b: usize = 0;
        for _ in 0..3 {
            if rng.gen_range(0, 100) < egg_move_chance {
                b += 1
            }
        }
        if b == 0 {
            return normal_moves
        }
        let mut rtrnval: Vec<usize> = egg_moves.into_iter().choose_multiple(&mut rng, b);
        rtrnval.append(&mut normal_moves.into_iter().choose_multiple(&mut rng, 4-b));
        return rtrnval
    }
    normal_moves.into_iter().choose_multiple(&mut rng, 4)
}

fn gen_rand_species(file_data: &[Pokemon], generation: &str) -> Pokemon {
    use rand::prelude::IteratorRandom;
    file_data.iter().filter(|x| {is_gen_lower_or_equal(x.pokemon_gen.as_str(), generation)}).choose(&mut rand::thread_rng()).unwrap().clone()
}

fn gen_rand_gender(species: &usize) -> u8 {
    use rand::Rng;
    let genderless_pokemon: [usize; 23] = [883, 881, 343, 374, 436, 703, 615, 781, 882, 880, 870, 622, 599, 337, 81, 774, 855, 137, 479, 338, 120, 201, 100];

    let female_only_pokemon: [usize; 12] = [29, 314, 440, 115, 238, 241, 548, 629, 669, 761, 856, 868];

    let male_only_pokemon: [usize; 7] = [32, 236, 128, 538, 539, 627, 859];

    if genderless_pokemon.contains(species) {
        2
    }
    else if female_only_pokemon.contains(species) {
        1
    }
    else if male_only_pokemon.contains(species) {
        0
    }
    else {
        rand::thread_rng().gen_range(0, 2)
    }
}

fn new_pokemon(file_data: &[Pokemon], game: &str, egg_move_chance: usize, hidden_ability_chance: usize, shiny_chance: usize, max_ivs: bool) -> PokemonStats {
    use rand::Rng;
    let generation: &str = game_to_gen(game);
    let pokemon: Pokemon = gen_rand_species(file_data, generation);
    let rand_moves = gen_rand_moves(&pokemon, game, egg_move_chance);
    let mut rng = rand::thread_rng();
    PokemonStats {
        Species: pokemon.pokemon_id,
        Ability: gen_rand_ability(&pokemon, generation, hidden_ability_chance),
        Gender: gen_rand_gender(&pokemon.pokemon_id),
        isShiny: shiny_chance > rng.gen_range(0, 100),
        Nature: rng.gen_range(0, 25),
        Hp: match max_ivs {
            true => 31,
            false => rng.gen_range(0, 31)
        },
        Atk: match max_ivs {
            true => 31,
            false => rng.gen_range(0, 31)
        },
        Def: match max_ivs {
            true => 31,
            false => rng.gen_range(0, 31)
        },
        SpA: match max_ivs {
            true => 31,
            false => rng.gen_range(0, 31)
        },
        SpD: match max_ivs {
            true => 31,
            false => rng.gen_range(0, 31)
        },
        Spe: match max_ivs {
            true => 31,
            false => rng.gen_range(0, 31)
        },
        moveOne: match rand_moves.get(0) {
            None => 0,
            Some(x) => *x
        },
        moveTwo: match rand_moves.get(1) {
            None => 0,
            Some(x) => *x
        },
        moveThree: match rand_moves.get(2) {
            None => 0,
            Some(x) => *x
        },
        moveFour: match rand_moves.get(3) {
            None => 0,
            Some(x) => *x
        }
    }
}

fn gen_pokemons(file_data: &[Pokemon], numb_to_gen: usize, game: String, egg_move_chance: usize, hidden_ability_chance: usize, shiny_chance: usize, maxivs: bool) -> String {
    serde_json::to_string::<Vec<PokemonStats>>((0..numb_to_gen).map( |_|
        new_pokemon(file_data, &game, egg_move_chance, hidden_ability_chance, shiny_chance, maxivs)
    ).collect::<Vec<PokemonStats>>().as_ref()).unwrap()
}

#[tokio::main]
async fn main() {
    let file_data: Vec<Pokemon> = serde_json::from_str(&FILE_DATA_GLOBAL).unwrap();
    let file_data2: Vec<Pokemon> = file_data.clone();
    let maxivs_route = warp::path!("maxivs" / usize / String / usize / usize / usize)
        .map(move |numb_to_gen, game, egg_move_chance, hidden_ability_chance, shiny_chance| gen_pokemons(&file_data, numb_to_gen, game, egg_move_chance, hidden_ability_chance, shiny_chance, true));
    let normal_route = warp::path!(usize / String / usize / usize / usize)
        .map(move |numb_to_gen, game, egg_move_chance, hidden_ability_chance, shiny_chance| gen_pokemons(&file_data2, numb_to_gen, game, egg_move_chance, hidden_ability_chance, shiny_chance, false));

    let socket = SocketAddr::new(IpAddr::from([127, 0, 0, 1]), env::var_os("PORT").unwrap().parse().unwrap());
    let routes = warp::get().and(maxivs_route.or(normal_route));
    warp::serve(routes).run(socket).await;
}
