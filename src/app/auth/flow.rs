use reqwest::Result;

use super::{pre_auth::fetch_pre_auth_data, initial_access_token::fetch_initial_access_token};

pub async fn fetch_cookies_and_authorization() -> Result<()> {
    let result = fetch_pre_auth_data().await;
    let another_result = fetch_initial_access_token(result).await?;
    println!("{:?}", another_result);
    Ok(())
}
