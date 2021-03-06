use crate::structures::{
    EmbedObject, InteractionReply, InteractionReplyData, InteractionReplyKind,
};
use serenity::model::prelude::ApplicationCommandInteractionDataOption;
use serenity::prelude::Context;

pub async fn avatar(
    client: &reqwest::Client,
    url: String,
    option_data: &ApplicationCommandInteractionDataOption,
    ctx: &Context,
    guild_id: u64,
) -> anyhow::Result<()> {
    let mut reply = InteractionReply {
        kind: InteractionReplyKind::CHANNEL_MESSAGE_WITH_SOURCE.0,
        data: InteractionReplyData {
            content: "".to_string(),
            embeds: None,
        },
    };
    if let Some(ref value) = option_data.value {
        let user_id = value.as_str().unwrap_or_default().parse::<u64>()?;
        if let Ok(member) = ctx.http.get_member(guild_id, user_id).await {
            if let Some(avatar_url) = member.user.avatar_url() {
                let member_name = member.nick.unwrap_or(member.user.name);
                let description = format!(
                    "Here is {}'s avatar!\n**[Avatar URL]({})**",
                    member_name.clone(),
                    avatar_url.clone()
                );

                let embed = EmbedObject::new()
                    .title(&member_name)
                    .description(&description)
                    .color(0xdfd16c)
                    .image(&avatar_url, None, None, None);

                reply.data = InteractionReplyData {
                    content: String::new(),
                    embeds: Some(vec![embed]),
                };
            } else {
                reply.data = InteractionReplyData {
                    content: "The user doesn't have an avatar!".to_string(),
                    embeds: None,
                };
            }
        } else {
            reply.data = InteractionReplyData {
                content: "Cannot find the user in this guild!".to_string(),
                embeds: None,
            }
        }

        let response = client.post(&url).json(&reply).send().await?;
        response.error_for_status()?;
    }

    Ok(())
}
