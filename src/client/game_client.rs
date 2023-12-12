use crate::dto::game_dto::GameDto;
use reqwest::Response;
use std::env;
use std::ops::Add;

const GAME_API: &str = "GAME_API";

pub struct GameClient {
    url: String,
}

impl GameClient {
    pub fn init() -> Self {
        let url = env::var(GAME_API).expect("Failed to load backend URL");
        GameClient { url }
    }

    pub async fn create_game(&self, game: GameDto) -> Response {
        let create_url = self.url.clone().add("/game");
        let client = reqwest::Client::new();
        let res = client.post(create_url).json(&game).send().await;
        res.expect("Failed to get result from create game client")
    }
}
