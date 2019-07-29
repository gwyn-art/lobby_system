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
use rocket::{
    State,
    response::status::NotFound
};
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

fn players_from_lobby(players_id: &mut Vec<Uuid>, players: &HashMap<Uuid, Player>) -> Vec<Player> {
    players_id
        .iter_mut()
        .map(|player_id| players.get(player_id).unwrap().clone())
        .collect()
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

#[post("/new/lobby", data = "<player>")]
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

#[post("/join/lobby/<code>", data = "<player>")]
fn lobby_join<'r>(
    code: String, 
    player: Json<PlayerNew>, 
    lobby_system: State<'r, LobbySystem>
) -> Result<JsonValue, NotFound<JsonValue>> {
    let mut lobbies = lobby_system.inner().lobbies.lock().unwrap();
    let option_lobby = lobbies.get_mut(&code);

    match option_lobby {
        Some(lobby) => {
            let mut players = lobby_system.inner().players.lock().unwrap();
            let new_player = Player {
                uuid: Uuid::new_v4(),
                name: player.name.clone(),
                role: PlayerRole::Default
            };

            lobby.players_id.push(new_player.uuid.clone());
            players.insert(new_player.uuid.clone(), new_player);

            Ok(json!({
                "code": lobby.code,
                "players": players_from_lobby(&mut lobby.players_id.clone(), &players)
            }))
        },
        None => {
            Err(NotFound(json!({"error": "Lobby not found."})))
        }
    }
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
            lobby_join,
            player_all
        ])
        .launch();
}