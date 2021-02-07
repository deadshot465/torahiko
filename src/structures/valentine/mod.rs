use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Valentine {
    pub birthday: String,
    pub color: String,
    pub name: String,
    pub description: String,
    pub animal: String,
    pub thumbnail_link: String,
    pub age: u8,
    pub zodiac: String,
}
