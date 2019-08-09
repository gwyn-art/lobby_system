#![feature(proc_macro_hygiene, decl_macro, rustc_private, associated_type_defaults, bind_by_move_pattern_guards)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate validator_derive;
extern crate rand;
extern crate uuid;
extern crate validator;

mod player;
mod lobby;
mod routes;
mod states;
mod utils;

use std::collections::HashMap;
use std::sync::RwLock;

use states::LobbyState;

fn main() {
    let mut rocket = rocket::ignite()
        .manage(
            LobbyState { 
                lobbies: RwLock::new(HashMap::new()),
                players: RwLock::new(HashMap::new())
            }
        );

    rocket = routes::player::mount(rocket);
    rocket = routes::lobby::mount(rocket);

    rocket.launch();
}