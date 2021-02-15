use crate::structures::{InteractionReply, InteractionReplyData, InteractionReplyKind};
use once_cell::sync::OnceCell;
use regex::Regex;
use serenity::model::prelude::ApplicationCommandInteractionDataOption;
use std::collections::HashMap;

const EMOTE_BASE_LINK: &str = "https://cdn.discordapp.com/emojis/";

static EMOTE_REGEX: OnceCell<Regex> = OnceCell::new();
static EMOTE_ID_REGEX: OnceCell<Regex> = OnceCell::new();
static EMOTE_IS_ANIMATED_REGEX: OnceCell<Regex> = OnceCell::new();

/// Returns an enlarged emote.
/// Parameter name: `emote`
pub async fn enlarge(
    client: &reqwest::Client,
    url: String,
    option_data: &ApplicationCommandInteractionDataOption,
    application_id: u64,
    interaction_token: String,
) -> anyhow::Result<()> {
    let mut reply = InteractionReply {
        kind: InteractionReplyKind::CHANNEL_MESSAGE_WITH_SOURCE.0,
        data: InteractionReplyData {
            content: "".to_string(),
            embeds: None,
        },
    };

    // Initialize or get regular expressions for emotes.
    // Since the initialization of regular expressions are usually expensive.
    // We use `OnceCell<Regex>` here to initialize them only once.
    let emote_regex = EMOTE_REGEX.get_or_init(|| {
        Regex::new(r"(<a?:\w+:\d+>)").expect("Failed to initialize regular expression.")
    });
    let emote_id_regex = EMOTE_ID_REGEX.get_or_init(|| {
        Regex::new(r"(:\w+:)(\d+)").expect("Failed to initialize regular expression.")
    });
    let emote_is_animated_regex = EMOTE_IS_ANIMATED_REGEX
        .get_or_init(|| Regex::new(r"(<a)").expect("Failed to initialize regular expression."));

    if let Some(ref value) = option_data.value {
        let emote_strings = value.as_str().unwrap_or_default();

        if !emote_id_regex.is_match(emote_strings) {
            reply.data.content = "There are no emotes in the input!".into();
            let response = client.post(&url).json(&reply).send().await?;
            response.error_for_status()?;
            return Ok(());
        }

        let split_emotes: Vec<&str> = emote_strings.split(' ').collect();
        let mut emote_links = vec![];

        for &emote in split_emotes.iter() {
            if !emote_regex.is_match(emote) {
                continue;
            }

            for capture in emote_regex.captures_iter(emote) {
                let emote_fullname = capture.get(1).expect("Failed to get emote name.");
                let id_capture = emote_id_regex
                    .captures(emote_fullname.as_str())
                    .expect("Failed to get emote ID from captured emote.");
                let id = id_capture
                    .get(2)
                    .expect("Failed to extract ID from capture.");

                emote_links.push(format!(
                    "{}{}{}",
                    EMOTE_BASE_LINK,
                    id.as_str(),
                    if emote_is_animated_regex.is_match(emote_fullname.as_str()) {
                        ".gif"
                    } else {
                        ".png"
                    }
                ));
            }
        }

        let followup_url = format!(
            "https://discord.com/api/webhooks/{}/{}",
            application_id, interaction_token
        );

        let mut data = HashMap::new();
        for (index, link) in emote_links.into_iter().enumerate() {
            match index {
                0 => {
                    reply.data.content = link;
                    let response = client.post(&url).json(&reply).send().await?;
                    response.error_for_status()?;
                }
                _ => {
                    let followup_data = data.entry("content").or_insert(link.clone());
                    *followup_data = link;
                    let response = client.post(&followup_url).json(&data).send().await?;
                    response.error_for_status()?;
                }
            }
        }
    }

    Ok(())
}
