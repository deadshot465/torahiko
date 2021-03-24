use crate::shared::create_new_followup_url;
use crate::structures::InteractionReply;
use serenity::{model::prelude::*, prelude::*};
use tokio::time::Duration;

pub async fn hangman(
    client: &reqwest::Client,
    url: String,
    member: &Member,
    application_id: u64,
    interaction_token: String,
    mut reply: InteractionReply,
) -> anyhow::Result<()> {
    let author_name = member.nick.clone().unwrap_or(member.user.name.clone());
    reply.data.content = format!("Hi, {}! We are going to play hangman!", &author_name);
    let mut response = client.post(&url).json(&reply).send().await?;
    response.error_for_status()?;

    let followup_url = create_new_followup_url(application_id, interaction_token);
    let mut followup_message = std::collections::HashMap::new();
    followup_message.insert("content", "Not yet completed...");
    tokio::time::sleep(Duration::from_secs(2)).await;
    response = client
        .post(&followup_url)
        .json(&followup_message)
        .send()
        .await?;

    println!("{}", response.text().await?);

    Ok(())
}
