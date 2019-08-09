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

#[post("/new", data = "<player>")]
fn new<'r>(
    player: Json<PlayerNew>, 
    lobby_state: State<'r, LobbyState>
) -> Result<JsonValue, BadRequest<JsonValue>> {
    let new_player = create_player(player.0, PlayerRole::Admin, &lobby_state)?;
    let new_lobby = create_lobby(new_player.uuid, &lobby_state);

    Ok(json!({
        "lobby": new_lobby,
        "creator": new_player
    }))
}

#[post("/join/<code>", data = "<player>")]
fn join<'r>(
    code: String, 
    player: Json<PlayerNew>, 
    lobby_state: State<'r, LobbyState>
) -> Result<JsonValue, BadRequest<JsonValue>> {
    let mut lobbies = lobby_state.inner().lobbies.write().unwrap();

    match lobbies.get_mut(&code) {
        Some(lobby) => {
            let new_player = 
                create_player(player.0, PlayerRole::Default, &lobby_state)?;

            lobby.players_id.push(new_player.uuid.clone());

            Ok(json!({
                "code": lobby.code,
                "players": players_from_lobby(
                    &mut lobby.players_id.clone(), 
                    &lobby_state.inner().players.read().unwrap()
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

#[get("/all")]
fn all<'r>(lobby_state: State<'r, LobbyState>) -> Json<Vec<Lobby>> {
    let lobbies = lobby_state.inner().lobbies.read().unwrap();
    let res = lobbies
        .values()
        .map(|lobby| lobby.clone())
        .collect();

    Json(res)
}

pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket
        .mount("/lobby", routes![all, new, join])
}