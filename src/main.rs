use dotenv::dotenv;
use lib::{application::Application, configuration};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let config = configuration::parse_config();
    println!("Run on port: {}", &config.app.port);

    Application::build(&config).await.run().await;
}
