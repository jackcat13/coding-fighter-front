use crate::dto::game_dto::GameDto;
use std::ops::Add;
extern crate dotenv;
use dotenv::dotenv;
use dotenv_codegen::dotenv;

pub struct GameClient {
    url: String,
}

#[allow(clippy::option_env_unwrap)]
impl GameClient {
    pub fn init() -> Self {
        dotenv().ok();
        let url = dotenv!("GAME_API").to_string();
        GameClient { url }
    }

    pub async fn create_game(&self, game: GameDto) -> GameDto {
        let create_url = self.url.clone().add("/game");
        let client = reqwest::Client::new();
        let res = client.post(create_url).json(&game).send().await;
        let res = res.expect("Failed to get result from create game client");
        let game_created: GameDto = res.json().await.expect("Failed to parse created game");
        game_created
    }

    pub async fn get_games(&self) -> Vec<GameDto> {
        let get_all_url = self.url.clone().add("/games");
        let client = reqwest::Client::new();
        let res = client.get(get_all_url).send().await;
        let res = res.expect("Failed to get result from get all games client");
        let games: Vec<GameDto> = res.json().await.expect("Failed to parse fetched games");
        games
    }
}
