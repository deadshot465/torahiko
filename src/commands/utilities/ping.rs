use crate::structures::interaction_replies::{
    InteractionReply, InteractionReplyData, InteractionReplyKind,
};
use std::collections::HashMap;
use std::time::Instant;

pub async fn ping(
    client: &reqwest::Client,
    url: String,
    application_id: u64,
    interaction_token: String,
) -> anyhow::Result<()> {
    let original_time = Instant::now();
    let reply = InteractionReply {
        kind: InteractionReplyKind::CHANNEL_MESSAGE_WITH_SOURCE.0,
        data: InteractionReplyData {
            content: "🏓 Pinging...".to_string(),
            embeds: None,
        },
    };

    let response = client.post(&url).json(&reply).send().await?;

    if let Err(err) = response.error_for_status() {
        log::error!(
            "Error when responding to slash command: {}",
            err.to_string()
        );
    } else {
        let current_time = Instant::now();
        let latency = current_time.duration_since(original_time);
        let patch_url = format!(
            "https://discord.com/api/webhooks/{}/{}/messages/@original",
            application_id, interaction_token
        );
        let mut reply_data = HashMap::new();
        reply_data.insert(
            "content",
            format!("🏓 Pong!\nLatency is: {}ms.", latency.as_millis()),
        );
        let response = client.patch(&patch_url).json(&reply_data).send().await?;

        if let Err(err) = response.error_for_status() {
            log::error!(
                "Error when responding to slash command: {}",
                err.to_string()
            );
        }
    }

    Ok(())
}
