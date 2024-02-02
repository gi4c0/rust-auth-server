use dotenv::dotenv;
use lib::{application::App, configuration};
use tracing::{info, Level};

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_target(false)
        .with_max_level(Level::INFO)
        .pretty()
        .init();

    let config = configuration::parse_config();
    info!("Listening on {}", &config.app.port);

    App::build(&config).await.run().await;
}
