use serde::{Deserialize, Serialize};

///AnswerDto is used to interact with the game frontend in the [GameResource](crate::resource::game_resource::GameResource)
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GameAnswerDto {
    pub game_id: String,
    pub user: String,
    pub answer: i8,
    pub question_index: i8,
    pub correct_answer: i8,
}
