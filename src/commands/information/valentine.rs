use crate::structures::{
    EmbedObject, InteractionReply, InteractionReplyData, InteractionReplyKind, Valentine,
};
use once_cell::sync::OnceCell;
use rand::prelude::*;
use serenity::model::prelude::Member;

const VALENTINE_FILE_PATH: &str = "./assets/valentines.json";
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

    let title = format!("Your valentine is {}", valentine.name.clone());
    let description = valentine.description.clone();
    let color = u32::from_str_radix(&valentine.color, 16)?;

    let first_name = get_first_name(&valentine.name);
    let valentine_age = valentine.age.to_string();
    let footer_text = format!("{} is adorable and his route is very nice, too. Maybe it's time to give it a try if you don't like his route.", first_name);
    let author_name = member.nick.clone().unwrap_or(member.user.name.clone());

    let embed = EmbedObject::new()
        .title(&title)
        .description(&description)
        .color(color)
        .footer(&footer_text, None, None)
        .thumbnail(&valentine.thumbnail_link, None, None, None)
        .author(&author_name, member.user.avatar_url(), None, None)
        .field("Age", &valentine_age, true)
        .field("Birthday", &valentine.birthday, true)
        .field("Animal", &valentine.animal, true)
        .field("Zodiac Sign", &valentine.zodiac, true);

    let reply = InteractionReply {
        kind: InteractionReplyKind::CHANNEL_MESSAGE_WITH_SOURCE.0,
        data: InteractionReplyData {
            content: String::new(),
            embeds: Some(vec![embed]),
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
