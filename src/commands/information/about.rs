use crate::structures::{
    EmbedObject, InteractionReply, InteractionReplyData, InteractionReplyKind,
};

pub async fn about(client: &reqwest::Client, url: String) -> anyhow::Result<()> {
    let description = "Ooshima Torahiko in the Land of Cute Bois.\nTorahiko was inspired by the visual novel Homecoming ~Morenatsu Revisited~.\n[Official website](https://stormsingerstudios.com/homecoming)\n[Official Discord server](https://discord.gg/WTjM3S5)\nTorahiko version 0.3 was made and developed by:\n**Tetsuki Syu#1250**"
        .to_string();

    let embed = EmbedObject::new()
        .description(&description)
        .color(0xdfd16c)
        .footer("Torahiko Bot: Release 0.3 | 2021-03-07", None, None)
        .thumbnail("https://cdn.discordapp.com/emojis/448579316171669545.png", None, None, None)
        .author("Ooshima Torahiko from Homecoming ~Morenatsu Revisited~", Some("https://cdn.discordapp.com/avatars/739040698342441030/1db851d49f1ff0f52c56dec4433cef30.webp?size=1024".into()), None, None);

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
