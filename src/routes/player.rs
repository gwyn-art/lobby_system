use crate::player::*;
use crate::LobbyState;

use rocket_contrib::json::{Json};
use rocket::{
    State
};

#[get("/all/player")]
pub fn player_all<'r>(lobby_state: State<'r, LobbyState>) -> Json<Vec<Player>> {
    let players = lobby_state.inner().players.read().unwrap();
    let res = players
        .values()
        .map(|player| player.clone())
        .collect();

    Json(res)
}