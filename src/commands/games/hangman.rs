use crate::commands::ONGOING_GAMES;
use crate::shared::create_new_followup_url;
use crate::structures::{GameRequest, GameRequestUser, InteractionReply};
use dashmap::DashMap;
use serenity::{model::prelude::*, prelude::*};
use std::sync::Arc;
use tokio::time::Duration;

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

    let data = GameRequest {
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
        .json(&data)
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
