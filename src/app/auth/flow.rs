use reqwest::Result;

use super::{
    authenticate::authenticate,
    authorize::{authorize, AuthorizeResult},
    initial_access_token::fetch_initial_access_token,
    pre_auth::fetch_pre_auth_data,
};

pub async fn fetch_cookies_and_authorization() -> Result<AuthorizeResult> {
    let pre_auth_payload = fetch_pre_auth_data().await;
    let initial_access_token_payload = fetch_initial_access_token(pre_auth_payload).await?;
    let authentication_result = authenticate(initial_access_token_payload).await?;
    let authorization_result = authorize(authentication_result).await?;
    Ok(authorization_result)
}
