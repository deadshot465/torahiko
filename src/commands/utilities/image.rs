use crate::shared::{get_cat_image, get_dog_image, get_image};
use crate::structures::{InteractionReply, InteractionReplyData, InteractionReplyKind};
use rand::{thread_rng, Rng};
use serenity::model::prelude::{ApplicationCommandInteractionDataOption, Member};

pub async fn image(
    client: &reqwest::Client,
    url: String,
    option_data: &[ApplicationCommandInteractionDataOption],
    member: &Member,
) -> anyhow::Result<()> {
    let mut reply = InteractionReply {
        kind: InteractionReplyKind::CHANNEL_MESSAGE_WITH_SOURCE.0,
        data: InteractionReplyData {
            content: "".to_string(),
            embeds: None,
        },
    };

    // Dispatch to different image services.
    // opt.name could be one of the followings: cat, dog, image
    if let Some(opt) = option_data.get(0) {
        let mut keyword = opt
            .options
            .get(0)
            .and_then(|a| {
                a.value
                    .clone()
                    .and_then(|v| v.as_str().map(|s| s.trim().to_lowercase()))
            })
            .unwrap_or_default();

        let result = match opt.name.as_str() {
            "image" => {
                if keyword.is_empty() {
                    keyword = "burger".into();
                }
                get_image(&keyword, client, member).await?
            }
            "cat" => {
                if thread_rng().gen_range(0..2) > 0 {
                    // Invoke the Cat API
                    get_cat_image(&keyword, client, member).await?
                } else {
                    // Invoke Unsplash API
                    keyword = if keyword.is_empty() {
                        "cat".into()
                    } else {
                        "cat ".to_string() + &keyword
                    };
                    get_image(&keyword, client, member).await?
                }
            }
            "dog" => {
                if thread_rng().gen_range(0..2) > 0 {
                    // Invoke Dog API
                    get_dog_image(&keyword, client, member).await?
                } else {
                    // Invoke Unsplash API
                    keyword = if keyword.is_empty() {
                        "dog".into()
                    } else {
                        "dog ".to_string() + &keyword
                    };
                    get_image(&keyword, client, member).await?
                }
            }
            _ => None,
        };

        if let Some(value) = result {
            reply.data.embeds = Some(vec![value]);
        } else {
            reply.data.content = format!("Sorry, I can't find any picture for **{}**!", &keyword);
        }

        let response = client.post(&url).json(&reply).send().await?;
        response.error_for_status()?;
    }
    Ok(())
}
