use crate::lobby::Lobby;
use crate::player::Player;

use uuid::Uuid;
use std::collections::HashMap;
use std::sync::RwLock;

pub struct LobbyState {
    pub lobbies: RwLock<HashMap<String, Lobby>>,
    pub players: RwLock<HashMap<Uuid, Player>>
}