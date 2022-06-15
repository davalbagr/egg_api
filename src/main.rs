#![allow(non_snake_case)]
#![warn(clippy::too_many_arguments)]
mod randgen;
use rand::rngs::ThreadRng;
use rand::thread_rng;
use std::net::{IpAddr, SocketAddr};
use warp::Filter;
use randgen::models::Pokemon;
use randgen::models::PokemonStats;
use randgen::gen_rand_pokemon;

const FILE_DATA_GLOBAL: &str = unsafe { std::str::from_utf8_unchecked(include_bytes!("pokemons.json")) };


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
    if numb_to_gen > 1000 {
        return "requested too many eggs to be generated".to_string();
    }
    serde_json::to_string::<Vec<PokemonStats>>(&(0..numb_to_gen).map(|_| {
        gen_rand_pokemon(
          file_data,
          &game,
          egg_move_chance,
          hidden_ability_chance,
          shiny_chance,
          maxivs,
          rng  
        )
    }).collect::<Vec<PokemonStats>>()).unwrap()
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
                &mut thread_rng(),
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
                &mut thread_rng(),
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
