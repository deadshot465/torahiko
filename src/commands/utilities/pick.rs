use crate::structures::{InteractionReply, InteractionReplyData, InteractionReplyKind};
use rand::prelude::*;
use rand::thread_rng;
use serenity::model::prelude::ApplicationCommandInteractionDataOption;
use std::collections::HashMap;

pub async fn pick(
    client: &reqwest::Client,
    url: String,
    option_data: &[ApplicationCommandInteractionDataOption],
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

    // Get total pick times from the first option.
    let times = if let Some(t) = option_data.get(0) {
        let value = t.value.clone().unwrap_or_default().as_u64().unwrap_or(1);
        if value < 1 {
            1
        } else {
            value
        }
    } else {
        1
    };

    // Get options to pick from the second option.
    let raw_string_options = if let Some(o) = option_data.get(1) {
        o.value
            .clone()
            .unwrap_or_default()
            .as_str()
            .unwrap_or_default()
            .trim()
            .to_string()
    } else {
        String::new()
    };

    if raw_string_options.is_empty() {
        reply.data.content = "You must provide options.".into();
        let response = client.post(&url).json(&reply).send().await?;
        response.error_for_status()?;
        return Ok(());
    }

    // Map raw string of each option to actual String options and sanitize.
    let options = raw_string_options
        .split("|")
        .filter_map(|s| {
            let trimmed = s.trim();
            if !trimmed.is_empty() {
                Some(trimmed)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    // Preemptively select from options if Tora only needs to pick one.
    if times == 1 {
        let result = {
            let mut rng = thread_rng();
            options.choose(&mut rng)
        };
        if let Some(&res) = result {
            reply.data.content = format!("<:tora_head:810582833083973672> | I pick **{}**!", res);
            let response = client.post(&url).json(&reply).send().await?;
            response.error_for_status()?;
            return Ok(());
        }
    }

    // Slash commands have to be replied in 3 seconds; otherwise, the token will be invalidated.
    // Therefore, we send an initial message first if Tora has to count for a while.
    reply.data.content = "Counting......".into();
    let response = client.post(&url).json(&reply).send().await?;
    response.error_for_status()?;

    let followup_url = format!(
        "https://discord.com/api/webhooks/{}/{}",
        application_id, interaction_token
    );

    // Clone and map each option to a HashMap.
    // The reason for this is that we need to use `thread_rng` to choose from options.
    let mut map = options
        .clone()
        .into_iter()
        .map(|s| (s, 0_u64))
        .collect::<HashMap<_, _>>();

    {
        // Thread-local random number generator cannot be passed beyond `await` safely.
        let mut rng = thread_rng();

        // Actually count for options.
        // We could use rayon here for paralleled counting, but do we really have that need?
        for _ in 0..times {
            if let Some(&opt) = options.choose(&mut rng) {
                let entry = map.entry(opt).or_insert(0);
                *entry += 1;
            }
        }
    }

    // Get the option that is picked most times.
    let result = map
        .iter()
        .max_by(|a, b| (*a).1.cmp((*b).1))
        .map(|pair| pair.0.to_string())
        .unwrap_or_default();
    let mut result_message = format!("<:tora_head:810582833083973672> | I pick **{}**!\n", result);

    // Consume and sort the map.
    let mut map = map.into_iter().map(|(s, t)| (s, t)).collect::<Vec<_>>();
    map.sort_by(|a, b| (*a).1.cmp(&(*b).1));
    map.reverse();

    // Consume and iterate through the map and list all results since we no longer need to use it afterwards.
    for (res, times) in map.into_iter() {
        result_message += &format!("⬤{} - {} times\n", res, times);
    }

    let mut response_message = HashMap::new();
    response_message.insert("content", result_message);
    let response = client
        .post(&followup_url)
        .json(&response_message)
        .send()
        .await?;
    response.error_for_status()?;
    return Ok(());
}
