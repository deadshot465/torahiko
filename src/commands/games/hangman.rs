use crate::shared::create_new_followup_url;
use crate::structures::{GameRequestData, InteractionReply};
use serenity::{model::prelude::*, prelude::*};
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

    let data = GameRequestData {
        kind: "Hangman".to_string(),
        user_id: member.user.id.0,
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

    Ok(())
}
