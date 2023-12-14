use crate::dto::game_dto::GameDto;
use reqwest::Response;
use std::ops::Add;
extern crate dotenv;
use dotenv::dotenv;

pub struct GameClient {
    url: String,
}

#[allow(clippy::option_env_unwrap)]
impl GameClient {
    pub fn init() -> Self {
        dotenv().ok();
        let url = option_env!("GAME_API")
            .expect("Failed to load backend URL")
            .to_string();
        GameClient { url }
    }

    pub async fn create_game(&self, game: GameDto) -> Response {
        let create_url = self.url.clone().add("/game");
        let client = reqwest::Client::new();
        let res = client.post(create_url).json(&game).send().await;
        res.expect("Failed to get result from create game client")
    }
}
