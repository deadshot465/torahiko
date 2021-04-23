use crate::structures::{InteractionReply, InteractionReplyData, InteractionReplyKind};
use dashmap::DashMap;
use once_cell::sync::OnceCell;
use serenity::{model::prelude::*, prelude::*};
use std::sync::Arc;

mod hangman;
pub use hangman::handle_hangman;

pub async fn games(
    client: &reqwest::Client,
    url: String,
    option_data: &[ApplicationCommandInteractionDataOption],
    member: &Member,
    application_id: u64,
    interaction_token: String,
    channel_id: u64,
) -> anyhow::Result<()> {
    let reply = InteractionReply {
        kind: InteractionReplyKind::CHANNEL_MESSAGE_WITH_SOURCE.0,
        data: InteractionReplyData {
            content: "".to_string(),
            embeds: None,
        },
    };

    // Dispatch to different game handlers.
    // opt.name could be one of the followings: quiz, hangman, tictactoe
    if let Some(opt) = option_data.get(0) {
        match opt.name.as_str() {
            "hangman" => {
                hangman::hangman(
                    client,
                    url,
                    member,
                    application_id,
                    interaction_token,
                    reply,
                    channel_id,
                )
                .await?
            }
            _ => (),
        }
    }

    Ok(())
}
