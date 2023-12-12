use serde::{Deserialize, Serialize};
use validator::Validate;

pub const CURRENT_GAME: &str = "currentGame";
pub const GAME_TOPICS: &str = "gameTopics";
pub const GAME_QUESTION_NUMBER: &str = "gameQuestionNumber";
pub const GAME_PRIVATE: &str = "gamePrivate";
pub const AVAILABLE_TOPICS: [&str; 3] = ["Rust", "Java", "Kotlin"];

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct GameSchema {
    #[validate(length(min = 1, message = "At least one topic is required."))]
    pub topics: Vec<String>,
    pub question_number: String,
}
