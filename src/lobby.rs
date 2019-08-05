use uuid::Uuid;
use rand::prelude::*;
use rocket::State;

use crate::states::LobbyState;

#[derive(Clone, Serialize, Deserialize)]
pub struct Lobby {
    pub code: String,
    pub players_id: Vec<Uuid>
}

pub fn create_code() -> String {
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

pub fn create_lobby<'r>(creator_id: Uuid, lobby_state: & State<'r, LobbyState>) -> Lobby {
    let code = create_code();
    let mut lobbies = lobby_state.inner().lobbies.lock().unwrap();

    let new_lobby = Lobby {
        code: code.clone(),
        players_id: vec![creator_id]
    };

    lobbies.insert(code.clone(), new_lobby.clone());

    new_lobby
}