use rand::{thread_rng, Rng};
use serde_derive::Deserialize;
use serde_json::{from_reader, Value};
use std::fs::File;

#[derive(Deserialize, Debug, Clone)]
struct Stats {
    hp: u16,
    attack: u16,
    defense: u16,
    spattack: u16,
    spdefense: u16,
    speed: u16,
}

#[derive(Deserialize, Debug, Clone)]
struct Pokemon {
    id: u16,
    name: String,
    types: Vec<u16>,
    stats: Stats,
    moves: Vec<u16>,
    weight: u16,
    height: u16,
}

#[derive(Debug)]
struct Team {
    pokemons: Vec<Pokemon>,
    name: String,
}

fn get_pokemons() -> Vec<Pokemon> {
    let file = File::open("pokemons.json").expect("file should open read only");
    let json: Value = from_reader(file).expect("file should be proper JSON");
    serde_json::from_value(json).unwrap()
}

fn pick_random_pokemon(pokemons: Vec<Pokemon>) -> Pokemon {
    let mut rng = thread_rng();
    let n = rng.gen_range(0, pokemons.len().to_owned());
    pokemons.get(n).unwrap().to_owned()
}

fn get_pokemons_for_team(pokemons: Vec<Pokemon>, amount_of_pokemons: u32) -> Vec<Pokemon> {
    let mut slice: Vec<Pokemon> = Vec::new();
    for _n in 0..amount_of_pokemons {
        let random_pokemon: Pokemon = pick_random_pokemon(pokemons.to_owned());
        slice.push(random_pokemon);
    }
    slice
}

fn main() {
    ascii_header();
    let amount_of_pokemons_per_team: u32 = 6;
    let pokemons = get_pokemons();
    let team1 = Team {
        pokemons: get_pokemons_for_team(pokemons.clone(), amount_of_pokemons_per_team),
        name: String::from("Team 1"),
    };
    let team2 = Team {
        pokemons: get_pokemons_for_team(pokemons, amount_of_pokemons_per_team),
        name: String::from("Team 2"),
    };

    println!("_________________________");
    println!("\n{}'s team:", team1.name);
    for p in team1.pokemons {
        println!("{}", p.name);
    }
    println!("_________________________");
    println!("\n{}'s team:", team2.name);
    for p in team2.pokemons {
        println!("{}", p.name);
    }
    println!("_________________________\n");
}

fn ascii_header() {
    println!(
        "
█▀█ █▀█ █▄▀ █▀▀ █▀▄▀█ █▀█ █▄░█   █▄▄ ▄▀█ ▀█▀ ▀█▀ █░░ █▀▀   █▀ █ █▀▄▀█ █░█ █░░ ▄▀█ ▀█▀ █▀█ █▀█
█▀▀ █▄█ █░█ ██▄ █░▀░█ █▄█ █░▀█   █▄█ █▀█ ░█░ ░█░ █▄▄ ██▄   ▄█ █ █░▀░█ █▄█ █▄▄ █▀█ ░█░ █▄█ █▀▄
"
    );
}
