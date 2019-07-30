use uuid::Uuid;
use std::collections::HashMap;

#[derive(Clone, Serialize)]
pub enum PlayerRole {
    Admin,
    Default
}

#[derive(Clone, Serialize)]
pub struct Player {
    pub uuid: Uuid,
    pub name: String,
    pub role: PlayerRole
}

pub fn players_from_lobby(
    players_id: &mut Vec<Uuid>,
    players: &HashMap<Uuid, Player>
) -> Vec<Player> {
    players_id
        .iter_mut()
        .map(|player_id| players.get(player_id).unwrap().clone())
        .collect()
}

#[derive(Clone, Deserialize)]
pub struct PlayerNew {
    pub name: String
}