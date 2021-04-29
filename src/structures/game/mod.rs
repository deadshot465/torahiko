use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GameRequest {
    #[serde(rename = "type")]
    pub kind: String,
    pub user: GameRequestUser,
    pub channel_id: u64,
    pub application_id: u64,
    pub interaction_token: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GameRequestUser {
    pub user_id: u64,
    pub nickname: String,
    pub avatar_url: String,
}

#[derive(Deserialize, Serialize, Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum GameStatus {
    InProgress,
    Stale,
    End,
}

#[derive(Deserialize, Serialize, Debug, Copy, Clone)]
pub struct GameProgressResponse {
    pub status: GameStatus,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum GameProgressRequest {
    Quiz {
        user_id: u64,
        message: String,
        channel_id: u64,
        guild_id: u64,
        message_id: u64,
    },
    Hangman {
        user_id: u64,
        message: String,
        channel_id: u64,
        guild_id: u64,
        message_id: u64,
    },
    Tictactoe {
        user_id: u64,
        message: String,
        channel_id: u64,
        guild_id: u64,
        message_id: u64,
    },
}
