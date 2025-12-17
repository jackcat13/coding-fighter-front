use serde::{Deserialize, Serialize};

///GameProgressDto is used to interact with the game backend in the [GameResource](crate::resource::game_resource::GameResource)
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GameProgressDto {
    pub current_question: i8,
    pub question_number: i8,
    pub question_content: QuestionDto,
}

///QuestionDto is used to interact with the game backend in the [GameResource](crate::resource::game_resource::GameResource)
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct QuestionDto {
    pub question_text: String,
    pub answer_1: String,
    pub answer_2: String,
    pub answer_3: String,
    pub answer_4: String,
    pub topic: String,
    pub remaining_time: u64,
}
