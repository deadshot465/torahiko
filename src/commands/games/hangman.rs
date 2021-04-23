use crate::shared::create_new_followup_url;
use crate::structures::{
    GameProgressRequest, GameProgressResponse, GameRequest, GameRequestUser, GameStatus,
    InteractionReply,
};
use dashmap::DashMap;
use once_cell::sync::OnceCell;
use serenity::{model::prelude::*, prelude::*};
use std::sync::Arc;
use tokio::time::Duration;

/// K: Channel ID, V: A vector of user IDs who are having games in that channel.
static ONGOING_GAMES: OnceCell<Arc<DashMap<u64, Vec<u64>>>> = OnceCell::new();

pub async fn hangman(
    client: &reqwest::Client,
    url: String,
    member: &Member,
    application_id: u64,
    interaction_token: String,
    mut reply: InteractionReply,
    channel_id: u64,
) -> anyhow::Result<()> {
    let author_name = member.nick.clone().unwrap_or(member.user.name.clone());
    reply.data.content = format!("Hi, {}! We are going to play hangman!", &author_name);
    let mut response = client.post(&url).json(&reply).send().await?;
    response.error_for_status()?;

    let request_data = GameRequest {
        kind: "Hangman".to_string(),
        user: GameRequestUser {
            user_id: member.user.id.0,
            nickname: member.nick.clone().unwrap_or(member.user.name.clone()),
            avatar_url: member
                .user
                .avatar_url()
                .unwrap_or(member.user.default_avatar_url()),
        },
        channel_id,
        application_id,
        interaction_token,
    };
    tokio::time::sleep(Duration::from_secs(2)).await;
    let endpoint = std::env::var("SERVER")?;
    response = client
        .post(endpoint + "/minigame/start")
        .json(&request_data)
        .send()
        .await?;

    {
        // Register the game to the dash map
        let ongoing_games = ONGOING_GAMES.get_or_init(|| Arc::new(DashMap::new()));
        let mut channel = ongoing_games.entry(channel_id).or_insert_with(Vec::new);
        channel.push(member.user.id.0);
    }

    Ok(())
}

pub async fn handle_hangman(
    client: &reqwest::Client,
    ctx: &Context,
    message: &Message,
) -> anyhow::Result<()> {
    let ongoing_games = ONGOING_GAMES.get_or_init(|| Arc::new(DashMap::new()));
    let mut game_stale = false;
    if let Some(gaming_members) = ongoing_games.get(&message.channel_id.0) {
        if gaming_members.contains(&message.author.id.0) {
            let request_data = GameProgressRequest::Hangman {
                user_id: message.author.id.0,
                message: message.content.clone(),
                channel_id: message.channel_id.0,
                guild_id: message.guild_id.map(|g| g.0).unwrap_or_default(),
                message_id: message.id.0,
            };
            let endpoint = std::env::var("SERVER")?;
            let response = client
                .post(endpoint + "/minigame/progress")
                .json(&request_data)
                .send()
                .await?;
            let game_status: GameProgressResponse = response.json().await?;
            game_stale = match game_status.status {
                GameStatus::InProgress => false,
                GameStatus::Stale => true,
            };
        }
    }

    if game_stale {
        if let Some(mut gaming_members) = ongoing_games.get_mut(&message.channel_id.0) {
            gaming_members.retain(|id| *id != message.author.id.0);
            message
                .reply(&ctx.http, "Game is stale and cancelled.")
                .await?;
        }
    }

    Ok(())
}
