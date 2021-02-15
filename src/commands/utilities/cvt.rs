use crate::structures::{
    ConversionTable, InteractionReply, InteractionReplyData, InteractionReplyKind,
};
use once_cell::sync::OnceCell;
use serenity::model::prelude::ApplicationCommandInteractionDataOption;

const CONVERSION_TABLE_PATH: &str = "./assets/conversion_table.json";
static CONVERSION_TABLE: OnceCell<ConversionTable> = OnceCell::new();

pub async fn cvt(
    client: &reqwest::Client,
    url: String,
    option_data: &[ApplicationCommandInteractionDataOption],
) -> anyhow::Result<()> {
    let conversion_table = CONVERSION_TABLE.get_or_init(|| {
        let raw_conversion_table =
            std::fs::read(CONVERSION_TABLE_PATH).expect("Failed to read conversion table.");
        serde_json::from_slice(&raw_conversion_table)
            .expect("Failed to deserialize conversion table.")
    });

    let mut reply = InteractionReply {
        kind: InteractionReplyKind::CHANNEL_MESSAGE_WITH_SOURCE.0,
        data: InteractionReplyData {
            content: "".to_string(),
            embeds: None,
        },
    };

    // Dispatch to different converters.
    if let Some(opt) = option_data.get(0) {
        let source_unit = opt
            .options
            .get(0)
            .and_then(|a| {
                a.value
                    .clone()
                    .and_then(|v| v.as_str().map(|s| s.trim().to_lowercase()))
            })
            .unwrap_or_default();
        let target_unit = opt
            .options
            .get(1)
            .and_then(|a| {
                a.value
                    .clone()
                    .and_then(|v| v.as_str().map(|s| s.trim().to_lowercase()))
            })
            .unwrap_or_default();
        let mut amount = opt
            .options
            .get(2)
            .and_then(|a| a.value.clone().and_then(|v| v.as_f64()))
            .unwrap_or_default();
        let origin_amount = amount;
        let mut result = match opt.name.as_str() {
            "length" => {
                if let Some(source) = conversion_table
                    .length
                    .get(&target_unit)
                    .and_then(|m| m.get(&source_unit))
                {
                    *source * amount
                } else {
                    0.0
                }
            }
            "weight" => {
                if let Some(source) = conversion_table
                    .weight
                    .get(&target_unit)
                    .and_then(|m| m.get(&source_unit))
                {
                    *source * amount
                } else {
                    0.0
                }
            }
            "temperature" => match target_unit.as_str() {
                "c" => {
                    if source_unit.as_str() == "f" {
                        amount -= 32.0;
                    } else if source_unit.as_str() == "k" {
                        amount -= 273.15;
                    }
                    if let Some(source) = conversion_table
                        .temperature
                        .get(&"c".to_string())
                        .and_then(|m| m.get(&source_unit))
                    {
                        *source * amount
                    } else {
                        0.0
                    }
                }
                "f" => {
                    let mut res = if let Some(source) = conversion_table
                        .temperature
                        .get(&"f".to_string())
                        .and_then(|m| m.get(&source_unit))
                    {
                        *source * amount
                    } else {
                        0.0
                    };
                    if source_unit.as_str() == "c" {
                        res += 32.0;
                    } else if source_unit.as_str() == "k" {
                        res -= 459.67;
                    }
                    res
                }
                "k" => {
                    if source_unit.as_str() == "c" {
                        amount += 273.15;
                    } else if source_unit.as_str() == "f" {
                        amount += 459.67;
                    }
                    if let Some(source) = conversion_table
                        .temperature
                        .get(&"k".to_string())
                        .and_then(|m| m.get(&source_unit))
                    {
                        *source * amount
                    } else {
                        0.0
                    }
                }
                _ => 0.0,
            },
            _ => 0.0,
        };
        result = (result * 1000.0).round() / 1000.0;
        reply.data.content = format!(
            "<:tora_face:810697357363511397> According to my calculations, {}{} is {}{}.",
            origin_amount,
            match source_unit.as_str() {
                "c" => "℃",
                "f" => "℉",
                "k" => "K",
                _ => &source_unit,
            },
            result,
            match target_unit.as_str() {
                "c" => "℃",
                "f" => "℉",
                "k" => "K",
                _ => &target_unit,
            }
        );
        let response = client.post(&url).json(&reply).send().await?;
        response.error_for_status()?;
    }
    Ok(())
}
