use dotenv::dotenv;
use lib::{application::Application, configuration};
use tracing::info;

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::fmt().with_target(true).pretty().init();

    let config = configuration::parse_config();
    info!("Listening on {}", &config.app.port);

    Application::build(&config).await.run().await;
}
