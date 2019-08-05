use rocket_contrib::json::{Json, JsonValue};
use rocket::{
    State,
    response::status::{
        BadRequest
    }
};

use crate::player::*;
use crate::lobby::*;
use crate::LobbyState;
use crate::player::create_player;

#[post("/new/lobby", data = "<player>")]
pub fn lobby_new<'r>(
    player: Json<PlayerNew>, 
    lobby_state: State<'r, LobbyState>
) -> Result<JsonValue, BadRequest<JsonValue>> {
    let code = create_code();
    let mut lobbies = lobby_state.inner().lobbies.lock().unwrap();
    let new_player = create_player(player.0, PlayerRole::Admin, &lobby_state)?;

    let new_lobby = Lobby {
        code: code.clone(),
        players_id: vec![new_player.uuid]
    };

    lobbies.insert(code.clone(), new_lobby.clone());

    Ok(json!({
        "lobby": new_lobby,
        "creator": new_player
    }))
    
}

#[post("/join/lobby/<code>", data = "<player>")]
pub fn lobby_join<'r>(
    code: String, 
    player: Json<PlayerNew>, 
    lobby_state: State<'r, LobbyState>
) -> Result<JsonValue, BadRequest<JsonValue>> {
    let mut lobbies = lobby_state.inner().lobbies.lock().unwrap();

    match lobbies.get_mut(&code) {
        Some(lobby) => {
            let new_player = 
                create_player(player.0, PlayerRole::Default, &lobby_state)?;

            lobby.players_id.push(new_player.uuid.clone());

            Ok(json!({
                "code": lobby.code,
                "players": players_from_lobby(
                    &mut lobby.players_id.clone(), 
                    &lobby_state.inner().players.lock().unwrap()
                )
            }))
        },
        None => {
            Err(BadRequest(Some(json!({
                "error": "Lobby not found.",
                "success": false
            }))))
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