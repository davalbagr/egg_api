#[path = "models.rs"] pub mod models;
use rand::rngs::ThreadRng;
use rand::prelude::*;
use rand::distributions::Uniform;
use models::Pokemon;
use models::PokemonStats;


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

fn gen_rand_ability(
    pokemon: &Pokemon,
    generation: &str,
    hidden_ability_chance: usize,
    rng: &mut ThreadRng,
) -> usize {
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

fn gen_rand_moves(
    pokemon: &Pokemon,
    game: &str,
    egg_move_chance: usize,
    rng: &mut ThreadRng,
) -> Vec<usize> {
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
    let mut moves = vec![];
    let mut amount_egg_moves = 0;
    for _ in 0..egg_moves.clone().take(4).count() {
        if rng.gen_range(0, 101) < egg_move_chance {
            amount_egg_moves += 1;
        }
    }
    moves.append(&mut egg_moves.choose_multiple(rng, amount_egg_moves));
    moves.append(&mut normal_moves.choose_multiple(rng, 4 - amount_egg_moves));
    moves
}




fn gen_rand_gender(species: &usize, rng: &mut ThreadRng) -> u8 {
    let genderless_pokemon: [usize; 23] = [
        883, 881, 343, 374, 436, 703, 615, 781, 882, 880, 870, 622, 599, 337, 81, 774, 855, 137,
        479, 338, 120, 201, 100,
    ];

    let female_only_pokemon: [usize; 12] =
        [29, 314, 440, 115, 238, 241, 548, 629, 669, 761, 856, 868];

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

pub fn gen_rand_pokemon(
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
        Hp: if max_ivs { 31 } else { rng.sample(range) },
        Atk: if max_ivs { 31 } else { rng.sample(range) },
        Def: if max_ivs { 31 } else { rng.sample(range) },
        SpA: if max_ivs { 31 } else { rng.sample(range) },
        SpD: if max_ivs { 31 } else { rng.sample(range) },
        Spe: if max_ivs { 31 } else { rng.sample(range) },
        moveOne: *rand_moves.get(0).unwrap_or(&0),
        moveTwo: *rand_moves.get(1).unwrap_or(&0),
        moveThree: *rand_moves.get(2).unwrap_or(&0),
        moveFour: *rand_moves.get(3).unwrap_or(&0),
    }
}
