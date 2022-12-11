use async_std;

use crate::app::auth::flow::fetch_cookies_and_authorization;
mod app;

#[async_std::main]
async fn main() {
    fetch_cookies_and_authorization().await.unwrap();
}
