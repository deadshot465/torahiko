use crate::commands::*;
use once_cell::sync::OnceCell;
use rand::prelude::*;
use serenity::model::prelude::{Activity, Interaction, Message, Ready};
use serenity::{async_trait, prelude::*};
use tokio::time::Duration;

const PRESENCES_PATH: &str = "./assets/presences.json";

static HTTP_CLIENT: OnceCell<reqwest::Client> = OnceCell::new();
static IS_HTTP_CLIENT_INITIALIZED: OnceCell<Mutex<bool>> = OnceCell::new();
static PRESENCES: OnceCell<Vec<String>> = OnceCell::new();

pub struct Handler;

impl Handler {
    async fn initialize_http_client() -> anyhow::Result<()> {
        if HTTP_CLIENT.get().is_none() {
            let client_initialized = IS_HTTP_CLIENT_INITIALIZED.get_or_init(|| Mutex::new(false));
            let mut initialized = client_initialized.lock().await;
            if !*initialized {
                let client = reqwest::Client::new();
                if HTTP_CLIENT.set(client).is_ok() {
                    *initialized = true;
                }
            }
        }
        Ok(())
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, new_message: Message) {
        let client = HTTP_CLIENT.get().expect("Failed to get http client.");

        // Handles hangman
        handle_hangman(client, &ctx, &new_message)
            .await
            .expect("Failed to send message to hangman handler.");
    }

    async fn ready(&self, ctx: Context, data_about_bot: Ready) {
        log::info!(
            "{}#{} is now online.",
            &data_about_bot.user.name,
            data_about_bot.user.discriminator
        );
        let presences = PRESENCES.get_or_init(|| {
            let raw_data =
                std::fs::read(PRESENCES_PATH).expect("Failed to read presences from JSON.");
            serde_json::from_slice(&raw_data).expect("Failed to deserialize from JSON.")
        });
        let presence = {
            let mut rng = thread_rng();
            presences.choose(&mut rng).cloned().unwrap_or_default()
        };
        ctx.set_activity(Activity::playing(&presence)).await;
        tokio::spawn(async move {
            let presences = presences;
            loop {
                tokio::time::sleep(Duration::from_secs(3600)).await;
                let presence = {
                    let mut rng = thread_rng();
                    presences.choose(&mut rng).cloned().unwrap_or_default()
                };
                ctx.set_activity(Activity::playing(&presence)).await;
            }
        });
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        let token = interaction.token.clone();
        let id = interaction.id.0;
        let guild_id = interaction.guild_id.0;
        let response_url = format!(
            "https://discord.com/api/v8/interactions/{}/{}/callback",
            id,
            token.clone()
        );
        let app_info = ctx
            .http
            .get_current_application_info()
            .await
            .expect("Failed to get application info.");
        let channel_id = interaction.channel_id.0;

        if let Some(ref data) = interaction.data {
            Self::initialize_http_client()
                .await
                .expect("Failed to initialize http client.");
            let client = HTTP_CLIENT.get().expect("Failed to get http client.");
            match data.name.as_str() {
                "about" => about(client, response_url)
                    .await
                    .expect("Failed to respond to /about command."),
                "avatar" => {
                    if let Some(option_data) = data.options.get(0) {
                        avatar(client, response_url, option_data, &ctx, guild_id)
                            .await
                            .expect("Failed to respond to /avatar command.");
                    }
                }
                "cvt" => {
                    cvt(client, response_url, &data.options)
                        .await
                        .expect("Failed to respond to /cvt command.");
                }
                "enlarge" => {
                    if let Some(option_data) = data.options.get(0) {
                        enlarge(client, response_url, option_data, app_info.id.0, token)
                            .await
                            .expect("Failed to respond to /enlarge command.");
                    }
                }
                "games" => {
                    games(
                        client,
                        response_url,
                        &data.options,
                        &interaction.member,
                        app_info.id.0,
                        token,
                        channel_id,
                    )
                    .await
                    .expect("Failed to respond to /games command.");
                }
                "image" => {
                    image(client, response_url, &data.options, &interaction.member)
                        .await
                        .expect("Failed to respond to /image command.");
                }
                "pick" => {
                    pick(client, response_url, &data.options, app_info.id.0, token)
                        .await
                        .expect("Failed to respond to /pick command.");
                }
                "ping" => {
                    ping(client, response_url, app_info.id.0, token)
                        .await
                        .expect("Failed to respond to /ping command.");
                }
                "valentine" => valentine(client, response_url, &interaction.member)
                    .await
                    .expect("Failed to respond to /valentine command."),
                _ => (),
            }
        }
    }
}
