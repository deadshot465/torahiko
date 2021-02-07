use crate::structures::embed::{EmbedAuthor, EmbedField, EmbedFooter, EmbedObject, EmbedThumbnail};
use crate::structures::interaction_replies::{
    InteractionReply, InteractionReplyData, InteractionReplyKind,
};
use crate::structures::valentine::Valentine;
use once_cell::sync::OnceCell;
use rand::prelude::*;
use serenity::model::prelude::Member;

const VALENTINE_FILE_PATH: &str = "./static/valentines.json";
static VALENTINES: OnceCell<Vec<Valentine>> = OnceCell::new();

pub async fn valentine(
    client: &reqwest::Client,
    url: String,
    member: &Member,
) -> anyhow::Result<()> {
    load_valentines()?;

    let valentine = {
        let valentines = VALENTINES.get().expect("Failed to get loaded valentines.");
        let mut rng = thread_rng();
        valentines
            .choose(&mut rng)
            .expect("Failed to choose from valentines.")
    };

    let author = EmbedAuthor {
        name: Some(member.nick.clone().unwrap_or(member.user.name.clone())),
        url: None,
        icon_url: member.user.avatar_url(),
        proxy_icon_url: None,
    };

    let title = format!("Your valentine is {}", valentine.name.clone());
    let description = valentine.description.clone();
    let color = u32::from_str_radix(&valentine.color, 16)?;

    let thumbnail = EmbedThumbnail {
        url: Some(valentine.thumbnail_link.clone()),
        proxy_url: None,
        height: None,
        width: None,
    };

    let fields = vec![
        EmbedField {
            name: "Age".to_string(),
            value: valentine.age.to_string(),
            inline: Some(true),
        },
        EmbedField {
            name: "Birthday".to_string(),
            value: valentine.birthday.clone(),
            inline: Some(true),
        },
        EmbedField {
            name: "Animal".to_string(),
            value: valentine.animal.clone(),
            inline: Some(true),
        },
        EmbedField {
            name: "Zodiac Sign".to_string(),
            value: valentine.zodiac.clone(),
            inline: Some(true),
        },
    ];

    let first_name = get_first_name(&valentine.name);

    let footer = EmbedFooter {
        text: format!("{} is adorable and his route is very nice, too. Maybe it's time to give it a try if you don't like his route.", first_name),
        icon_url: None,
        proxy_icon_url: None,
    };

    let reply = InteractionReply {
        kind: InteractionReplyKind::CHANNEL_MESSAGE_WITH_SOURCE.0,
        data: InteractionReplyData {
            content: String::new(),
            embeds: Some(vec![EmbedObject {
                title: Some(title),
                kind: None,
                description: Some(description),
                url: None,
                color: Some(color),
                footer: Some(footer),
                image: None,
                thumbnail: Some(thumbnail),
                video: None,
                provider: None,
                author: Some(author),
                fields: Some(fields),
            }]),
        },
    };

    let response = client.post(&url).json(&reply).send().await?;
    response.error_for_status()?;

    Ok(())
}

fn load_valentines() -> anyhow::Result<()> {
    if VALENTINES.get().is_some() {
        return Ok(());
    }

    let raw_bytes = std::fs::read(VALENTINE_FILE_PATH)?;
    VALENTINES
        .set(serde_json::from_slice(&raw_bytes)?)
        .unwrap_or(());

    Ok(())
}

fn get_first_name(name: &str) -> &str {
    let first_name: Vec<&str> = name.split(' ').collect();
    first_name[1]
}
