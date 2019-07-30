use uuid::Uuid;
use rand::prelude::*;

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