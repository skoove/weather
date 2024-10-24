use reqwest::Error;
use tokio;

mod location;
mod ui;
mod weather;

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("weather!!");

    Ok(())
}
