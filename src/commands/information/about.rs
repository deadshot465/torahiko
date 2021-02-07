use crate::structures::embed::{EmbedAuthor, EmbedFooter, EmbedObject, EmbedThumbnail};
use crate::structures::interaction_replies::{
    InteractionReply, InteractionReplyData, InteractionReplyKind,
};

pub async fn about(client: &reqwest::Client, url: String) -> anyhow::Result<()> {
    let description = "Ooshima Torahiko in The Church of Minamoto Kou.\nTorahiko was inspired by the visual novel Homecoming ~Morenatsu Revisited~.\n[Official website](https://stormsingerstudios.com/homecoming)\n[Official Discord server](https://discord.gg/WTjM3S5)\nTorahiko version 0.1 was made and developed by:\n**Tetsuki Syu#1250**"
        .to_string();

    let reply = InteractionReply {
        kind: InteractionReplyKind::CHANNEL_MESSAGE_WITH_SOURCE.0,
        data: InteractionReplyData {
            content: String::new(),
            embeds: Some(vec![EmbedObject {
                title: None,
                kind: None,
                description: Some(description),
                url: None,
                color: Some(0xdfd16c),
                footer: Some(EmbedFooter {
                    text: "Torahiko Bot: Release 0.1 | 2021-02-07".to_string(),
                    icon_url: None,
                    proxy_icon_url: None,
                }),
                image: None,
                thumbnail: Some(EmbedThumbnail {
                    url: Some(
                        "https://cdn.discordapp.com/emojis/448579316171669545.png".to_string(),
                    ),
                    proxy_url: None,
                    height: None,
                    width: None,
                }),
                video: None,
                provider: None,
                author: Some(EmbedAuthor {
                    name: Some(
                        "Ooshima Torahiko from Homecoming ~Morenatsu Revisited~".to_string(),
                    ),
                    url: None,
                    icon_url: Some("https://cdn.discordapp.com/avatars/739040698342441030/1db851d49f1ff0f52c56dec4433cef30.webp?size=1024".to_string()),
                    proxy_icon_url: None,
                }),
                fields: None,
            }]),
        },
    };

    let response = client.post(&url).json(&reply).send().await?;

    if let Err(err) = response.error_for_status() {
        log::error!(
            "Error when responding to slash command: {}",
            err.to_string()
        );
    }

    Ok(())
}
