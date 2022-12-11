use reqwest::Result;

use super::{
    authenticate::authenticate, authorize::authorize,
    initial_access_token::fetch_initial_access_token, pre_auth::fetch_pre_auth_data,
    utils::turn_cookie_vector_to_string,
};

pub struct EntireAuthorizationFlowResult {
    pub cookie: String,
    pub authorization_header: String,
}

pub async fn fetch_cookies_and_authorization() -> Result<EntireAuthorizationFlowResult> {
    let pre_auth_payload = fetch_pre_auth_data().await;
    let initial_access_token_payload = fetch_initial_access_token(pre_auth_payload).await?;
    let authentication_result = authenticate(initial_access_token_payload).await?;
    let authorization_result = authorize(authentication_result).await?;
    Ok(EntireAuthorizationFlowResult {
        cookie: turn_cookie_vector_to_string(&authorization_result.cookies),
        authorization_header: authorization_result.authorization_header,
    })
}
