#![feature(proc_macro_hygiene, decl_macro, rustc_private)]

#[macro_use] extern crate rocket;
extern crate rand;

use rand::prelude::*;
use std::collections::HashMap;
use std::sync::Mutex;
use rocket::State;

#[derive(Clone)]
struct Lobby {
    pub code: String
}

struct LobbySystem {
    pub lobbies: Mutex<HashMap<String, Lobby>>
}

fn create_code() -> String {
    let mut rng = rand::thread_rng();
    let mut code = [0; 6];

    for x in code.iter_mut() {
        *x = rng.gen_range(0, 10);
    }

    code
        .into_iter()
        .map(|x| x.to_string())
        .collect()
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/new/lobby")]
fn lobby_new<'r>(memory: State<'r, LobbySystem>) -> String {
    let code = create_code();
    
    let mut new_lobby = memory.inner().lobbies.lock().unwrap();
    new_lobby.insert(code.clone(), Lobby {
        code: code.clone()
    });

    format!("Your code is: {}", code)
}

#[get("/all/lobby")]
fn lobby_all<'r>(memory: State<'r, LobbySystem>) -> String {
    let lobby = memory.inner().lobbies.lock().unwrap();
    let mut res = String::new();

    for x in lobby.iter() {
        res = format!("{}{}\n", res, x.0);
    }

    format!("All games: \n{}", res)
}

fn main() {
    rocket
        ::ignite()
        .manage(LobbySystem { lobbies: Mutex::new(HashMap::new())})
        .mount("/", routes![
            index, 
            lobby_new,
            lobby_all
        ])
        .launch();
}