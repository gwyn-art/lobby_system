use crate::player::*;
use crate::lobby::*;
use crate::LobbyState;

use rocket_contrib::json::{Json, JsonValue};
use rocket::{
    State,
    response::status::NotFound
};
use uuid::Uuid;

#[post("/new/lobby", data = "<player>")]
pub fn lobby_new<'r>(player: Json<PlayerNew>, lobby_state: State<'r, LobbyState>) -> Option<JsonValue> {
    let code = create_code();
    let mut lobbies = lobby_state.inner().lobbies.lock().unwrap();
    let mut players = lobby_state.inner().players.lock().unwrap();

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
pub fn lobby_join<'r>(
    code: String, 
    player: Json<PlayerNew>, 
    lobby_state: State<'r, LobbyState>
) -> Result<JsonValue, NotFound<JsonValue>> {
    let mut lobbies = lobby_state.inner().lobbies.lock().unwrap();
    let option_lobby = lobbies.get_mut(&code);

    match option_lobby {
        Some(lobby) => {
            let mut players = lobby_state.inner().players.lock().unwrap();
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
pub fn lobby_all<'r>(lobby_state: State<'r, LobbyState>) -> Json<Vec<Lobby>> {
    let lobbies = lobby_state.inner().lobbies.lock().unwrap();
    let res = lobbies
        .values()
        .map(|lobby| lobby.clone())
        .collect();

    Json(res)
}