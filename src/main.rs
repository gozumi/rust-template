use app_config::AppConfig;
use dotenv::dotenv;
use std::error::Error;

mod app_config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let app_config = AppConfig::new();

    println!("{:#?}", app_config);

    Ok(())
}
