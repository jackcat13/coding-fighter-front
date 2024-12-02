extern crate dotenv;

use std::ops::Add;

use dotenv::dotenv;
use dotenv_codegen::dotenv;
use reqwest::header::CONTENT_TYPE;
use serde_json::json;

use crate::{
    dto::{answer::GameAnswerDto, game_dto::GameDto},
    helpers::local_storage::local_storage,
    USER_SESSION,
};

const APPLICATION_JSON: &str = "application/json";

pub struct GameClient {
    url: String,
}

/// Client to interact with the game backend
/// It provides methods to create, get all and get a game by id
#[allow(clippy::option_env_unwrap)]
impl GameClient {
    pub fn init() -> Self {
        dotenv().ok();
        let url = dotenv!("GAME_API").to_string();
        GameClient { url }
    }

    pub async fn create_game(&self, game: GameDto) -> GameDto {
        gloo::console::log!("Calling the client to create game");
        let create_url = self.url.clone().add("/game");
        let client = reqwest::Client::new();
        let res = client.post(create_url).json(&game).send().await;
        let res = res.expect("Failed to get result from create game client");
        let game_created: GameDto = res.json().await.expect("Failed to parse created game");
        gloo::console::log!("Returning the created game");
        game_created
    }

    pub async fn get_games(&self) -> Vec<GameDto> {
        gloo::console::log!("Calling the client to get games");
        let get_all_url = self.url.clone().add("/games");
        let client = reqwest::Client::new();
        let res = client.get(get_all_url).send().await;
        let res = res.expect("Failed to get result from get all games client");
        let games: Vec<GameDto> = res.json().await.expect("Failed to parse fetched games");
        gloo::console::log!("Returning the games");
        games
    }

    pub async fn get_game(&self, id: String) -> Option<GameDto> {
        gloo::console::log!("Calling the client to get game by id");
        let get_url = self.url.clone().add("/game/").add(id.as_str());
        let client = reqwest::Client::new();
        let res = client.get(get_url).send().await;
        let res = res.expect("Failed to get result from client to get game by id");
        let game: Option<GameDto> = res.json().await.expect("Failed to parse fetched game");
        gloo::console::log!("Returning the game");
        game
    }

    pub async fn start_game(&self, id: String) {
        gloo::console::log!("Calling the client to start game by id");
        let get_url = self.url.clone().add("/game/").add(id.as_str());
        let client = reqwest::Client::new();
        let res = client
            .patch(get_url)
            .header(CONTENT_TYPE, APPLICATION_JSON)
            .send()
            .await;
        let _ = res.expect("Failed to get result from client to start game by id");
        gloo::console::log!("Game should be started");
    }

    pub async fn send_answer(&self, id: String, answer: i8) {
        gloo::console::log!("Calling the client to send answer");
        let storage = local_storage();
        let user = storage
            .get_item(USER_SESSION)
            .expect("Failed to get user from local storage");
        let get_url = self
            .url
            .clone()
            .add("/game/")
            .add(id.as_str())
            .add("/progress/")
            .add(answer.to_string().as_str());

        let client = reqwest::Client::new();
        let res = client
            .post(get_url)
            .header(CONTENT_TYPE, APPLICATION_JSON)
            .json(&json!({"user": user}))
            .send()
            .await;
        let _ = res.expect("Failed to get result from client to send answer");
        gloo::console::log!("Answer should be sent");
    }

    pub async fn get_game_answers(&self, game_id: String) -> Vec<GameAnswerDto> {
        gloo::console::log!("Calling the client to get game answers");
        let get_answers_url = self
            .url
            .clone()
            .add("/game/")
            .add(game_id.as_str())
            .add("/answers");
        let client = reqwest::Client::new();
        let res = client.get(get_answers_url).send().await;
        let res = res.expect("Failed to get answers from get answers client");
        let answers: Vec<GameAnswerDto> =
            res.json().await.expect("Failed to parse fetched answers");
        gloo::console::log!("Returning the answers");
        answers
    }

    pub fn progress_events_souce_url(&self, id: &str) -> String {
        String::from(&self.url.clone().add("/game/").add(id).add("/progress"))
    }
}
