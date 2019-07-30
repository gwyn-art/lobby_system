use crate::lobby::Lobby;
use crate::player::Player;

use uuid::Uuid;
use std::collections::HashMap;
use std::sync::Mutex;

pub struct LobbyState {
    pub lobbies: Mutex<HashMap<String, Lobby>>,
    pub players: Mutex<HashMap<Uuid, Player>>
}