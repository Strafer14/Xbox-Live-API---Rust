use async_std;

use crate::app::auth::flow::fetch_cookies_and_authorization;
use dotenv::dotenv;
mod app;

#[async_std::main]
async fn main() {
    dotenv().ok();
    fetch_cookies_and_authorization().await.unwrap();
}
