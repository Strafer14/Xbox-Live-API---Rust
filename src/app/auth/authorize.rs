use reqwest::header::COOKIE;
use serde::{Serialize};

use super::{authenticate::{AuthenticationResult, AuthenticationResponse}, utils::turn_cookie_vector_to_string};

#[derive(Serialize)]
pub struct Properties {
    UserTokens: Vec<String>,
    SandboxId: String,
}

#[derive(Serialize)]
pub struct AuthorizePayload {
    RelyingParty: String,
    TokenType: String,
    Properties: Properties,
}

pub struct AuthorizeResult {
    pub cookies: Vec<(String, String)>,
    pub authorization_header: String,
}

pub async fn authorize(authentication_data: AuthenticationResult) -> reqwest::Result<AuthorizeResult> {
    let client = reqwest::Client::new();
    let properties = Properties {
        UserTokens: [authentication_data.token].to_vec(),
        SandboxId: "RETAIL".to_owned(),
    };
    let payload = AuthorizePayload {
        RelyingParty: "http://xboxlive.com".to_owned(),
        TokenType: "JWT".to_owned(),
        Properties: properties,
    };
    let cookie_string = turn_cookie_vector_to_string(&authentication_data.cookies);
    let request_url = "https://xsts.auth.xboxlive.com/xsts/authorize";
    let access_token_response: AuthenticationResponse = client
        .post(request_url)
        .json(&payload)
        .header(COOKIE, cookie_string)
        .send()
        .await?
        .json()
        .await?;
    let token = &access_token_response.Token;
    let user_hash = &access_token_response.DisplayClaims.xui[0].uhs;
    let authorization_header = format!("XBL3.0 x=${user_hash};${token}");
    return Ok(AuthorizeResult {
        cookies: authentication_data.cookies,
        authorization_header,
    })
}
