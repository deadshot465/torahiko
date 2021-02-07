use env_logger::Builder;
use log::LevelFilter;
use serenity::client::bridge::gateway::GatewayIntents;
use serenity::client::ClientBuilder;
use serenity::framework::StandardFramework;
use serenity::http::Http;
use std::collections::HashSet;
use torahiko::event_handler::Handler;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    Builder::new()
        .filter(None, LevelFilter::Warn)
        .default_format()
        .init();
    let token = dotenv::var("TOKEN")?;
    let http = Http::new_with_token(&token);
    let app_info = http.get_current_application_info().await?;
    let mut owners = HashSet::new();
    owners.insert(app_info.owner.id);

    let framework = StandardFramework::new().configure(|c| c.owners(owners));

    let mut client = ClientBuilder::new(token)
        .framework(framework)
        .event_handler(Handler)
        .intents(GatewayIntents::privileged())
        .intents(GatewayIntents::non_privileged())
        .await?;

    if let Err(reason) = client.start().await {
        log::error!(
            "An error occurred when starting the client: {}",
            reason.to_string()
        );
        Err(anyhow::anyhow!(
            "An error occurred when starting the client."
        ))
    } else {
        Ok(())
    }
}
