use crate::structures::{InteractionReply, InteractionReplyData, InteractionReplyKind};
use serenity::model::prelude::ApplicationCommandInteractionDataOption;

pub async fn image(
    _client: &reqwest::Client,
    _url: String,
    option_data: &ApplicationCommandInteractionDataOption,
) -> anyhow::Result<()> {
    /*let mut reply = InteractionReply {
        kind: InteractionReplyKind::CHANNEL_MESSAGE_WITH_SOURCE.0,
        data: InteractionReplyData {
            content: "".to_string(),
            embeds: None,
        },
    };*/

    if let Some(ref keyword) = option_data.value {}

    Ok(())
}
