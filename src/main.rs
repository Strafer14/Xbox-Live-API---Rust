use async_std;

use crate::app::api::fetch_profile;
use dotenv::dotenv;
mod app;

#[async_std::main]
async fn main() {
    dotenv().ok();
    let profile = fetch_profile("Daaaavie").await;
    println!("{:#?}", profile.unwrap());
}
