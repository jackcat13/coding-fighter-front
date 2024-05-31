use serde::{Deserialize, Serialize};

///GameDto is used to interact with the game backend by the [GameClient](crate::client::game_client::GameClient)
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GameDto {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub topics: Vec<String>,
    pub question_number: i8,
    pub is_private: bool,
    pub is_started: bool,
    pub creator: Option<String>,
}
