use app_config::AppConfig;
use dotenv::dotenv;
use std::{env, error::Error};
use store::results::Results;

mod app_config;
mod store;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let mongo_user = env::var("MONGO_USER")?;

    println!("mongo_user: {:?}", mongo_user);

    let app_config = AppConfig::new();

    println!("{:#?}", app_config);

    let r = Results::new();
    let s = r.bar;

    println!("{}", s);

    Ok(())
}
