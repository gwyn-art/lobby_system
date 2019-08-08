use uuid::Uuid;
use std::collections::{HashMap};
use validator::{Validate};
use rocket_contrib::json::JsonValue;
use rocket::{
    State,
    response::status::BadRequest
};

use crate::utils::errors_to_map; 
use crate::states::LobbyState;

#[derive(Debug, Clone, Serialize)]
pub enum PlayerRole {
    Admin,
    Default
}

#[derive(Debug, Clone, Serialize)]
pub struct Player {
    pub uuid: Uuid,
    pub name: String,
    pub role: PlayerRole
}


#[derive(Clone, Validate, Deserialize)]
pub struct PlayerNew {
    #[validate(length(min = 2, max = 12, message = "Player name should be between 2 and 12 chars."))]
    pub name: String
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

pub fn create_player<'r>(
    player: PlayerNew, 
    role: PlayerRole, 
    lobby_state: & State<'r, LobbyState>
) -> Result<Player, BadRequest<JsonValue>> {
    match player.validate() {
        Ok(_) => {
            let new_player = Player {
                name: player.name.clone(),
                uuid: Uuid::new_v4(),
                role: role
            };

            let mut players = lobby_state.inner().players.write().unwrap();
            players.insert(new_player.uuid.clone(), new_player.clone());

            Ok(new_player)
        }
        Err(errors) => {
            Err(BadRequest(Some(json!(
                {
                    "success": false,
                    "err": errors_to_map(&errors)
                }
            ))))
        }
    }
} 