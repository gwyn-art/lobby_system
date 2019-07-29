#![feature(proc_macro_hygiene, decl_macro, rustc_private)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
extern crate rocket_contrib;
extern crate rand;

use rand::prelude::*;
use std::collections::HashMap;
use std::sync::Mutex;

use rocket_contrib::json::{Json};
use rocket::State;

#[derive(Clone, Serialize, Deserialize)]
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
fn lobby_new<'r>(lobby_system: State<'r, LobbySystem>) -> Option<Json<Lobby>> {
    let code = create_code();
    
    let mut lobbies = lobby_system.inner().lobbies.lock().unwrap();
    let new_lobby = Lobby {
        code: code.clone()
    };

    lobbies.insert(code.clone(), new_lobby.clone());

    Some(Json(new_lobby))
}

#[get("/all/lobby")]
fn lobby_all<'r>(lobby_system: State<'r, LobbySystem>) -> Option<Json<Vec<Lobby>>> {
    let lobbies = lobby_system.inner().lobbies.lock().unwrap();
    let res = lobbies
        .values()
        .map(|lobby| lobby.clone())
        .collect::<Vec<Lobby>>();

    Some(Json(res))
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