#![feature(proc_macro_hygiene, decl_macro, rustc_private)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rocket_contrib;
extern crate rand;
extern crate uuid;

use rand::prelude::*;
use std::collections::HashMap;
use std::sync::Mutex;

use rocket_contrib::json::{Json, JsonValue};
use rocket::State;
use uuid::Uuid;

#[derive(Clone, Serialize)]
enum PlayerRole {
    Admin,
    Default
}

#[derive(Clone, Serialize)]
struct Player {
    pub uuid: Uuid,
    pub name: String,
    pub role: PlayerRole
}

#[derive(Clone, Deserialize)]
struct PlayerNew {
    pub name: String
}

#[derive(Clone, Serialize, Deserialize)]
struct Lobby {
    pub code: String,
    pub players_id: Vec<Uuid>
}

struct LobbySystem {
    pub lobbies: Mutex<HashMap<String, Lobby>>,
    pub players: Mutex<HashMap<Uuid, Player>>
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

#[post("/new/lobby", format = "json", data = "<player>")]
fn lobby_new<'r>(player: Json<PlayerNew>, lobby_system: State<'r, LobbySystem>) -> Option<JsonValue> {
    let code = create_code();
    let mut lobbies = lobby_system.inner().lobbies.lock().unwrap();
    let mut players = lobby_system.inner().players.lock().unwrap();

    let new_player = Player {
        name: player.name.clone(),
        uuid: Uuid::new_v4(),
        role: PlayerRole::Admin
    };

    let new_lobby = Lobby {
        code: code.clone(),
        players_id: vec![new_player.uuid]
    };

    lobbies.insert(code.clone(), new_lobby.clone());
    players.insert(new_player.uuid, new_player.clone());

    Some(json!({
        "lobby": new_lobby,
        "creator": new_player
    }))
    
}

#[get("/all/lobby")]
fn lobby_all<'r>(lobby_system: State<'r, LobbySystem>) -> Json<Vec<Lobby>> {
    let lobbies = lobby_system.inner().lobbies.lock().unwrap();
    let res = lobbies
        .values()
        .map(|lobby| lobby.clone())
        .collect();

    Json(res)
}

#[get("/all/player")]
fn player_all<'r>(lobby_system: State<'r, LobbySystem>) -> Json<Vec<Player>> {
    let players = lobby_system.inner().players.lock().unwrap();
    let res = players
        .values()
        .map(|player| player.clone())
        .collect();

    Json(res)
}

fn main() {
    rocket
        ::ignite()
        .manage(
            LobbySystem { 
                lobbies: Mutex::new(HashMap::new()),
                players: Mutex::new(HashMap::new())
            }
        )
        .mount("/", routes![
            lobby_new,
            lobby_all,
            player_all
        ])
        .launch();
}