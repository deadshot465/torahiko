use crate::structures::embed::EmbedObject;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct InteractionReply {
    #[serde(rename = "type")]
    pub kind: u8,
    pub data: InteractionReplyData,
}

#[derive(Deserialize, Serialize)]
pub struct InteractionReplyData {
    pub content: String,
    pub embeds: Option<Vec<EmbedObject>>,
}

pub struct InteractionReplyKind(pub(crate) u8);

impl InteractionReplyKind {
    pub const PONG: Self = Self(1);
    pub const ACKNOWLEDGE: Self = Self(2);
    pub const CHANNEL_MESSAGE: Self = Self(3);
    pub const CHANNEL_MESSAGE_WITH_SOURCE: Self = Self(4);
    pub const ACKNOWLEDGE_WITH_SOURCE: Self = Self(5);
}
