use reqwest::header::COOKIE;
use serde::{Deserialize, Serialize};

use super::{
    initial_access_token::FetchInitialTokenResultData, utils::turn_cookie_vector_to_string,
};

#[derive(Serialize)]
pub struct AuthenticatePayload {
    RelyingParty: String,
    TokenType: String,
    Properties: Properties,
}

#[derive(Serialize)]
pub struct Properties {
    AuthMethod: String,
    SiteName: String,
    RpsTicket: String,
}

#[derive(Deserialize, Serialize)]
pub struct Xui {
    pub uhs: String,
}

#[derive(Deserialize, Serialize)]
pub struct DisplayClaims {
    pub xui: Vec<Xui>,
}

#[derive(Deserialize, Serialize)]
pub struct AuthenticationResponse {
    NotAfter: String,
    pub Token: String,
    pub DisplayClaims: DisplayClaims,
}

pub struct AuthenticationResult {
    pub token: String,
    user_hash: String,
    not_after: String,
    pub cookies: Vec<(String, String)>,
}

pub async fn authenticate(payload: FetchInitialTokenResultData) -> reqwest::Result<AuthenticationResult> {
    let client = reqwest::Client::new();
    let cookies = payload.cookies;
    let access_token = payload.access_token;
    let payload = AuthenticatePayload {
        RelyingParty: "http://auth.xboxlive.com".to_owned(),
        TokenType: "JWT".to_owned(),
        Properties: Properties {
            AuthMethod: "RPS".to_owned(),
            SiteName: "user.auth.xboxlive.com".to_owned(),
            RpsTicket: access_token,
        },
    };
    let request_url = "https://user.auth.xboxlive.com/user/authenticate";
    let cookie_string = turn_cookie_vector_to_string(&cookies);
    let access_token_response: AuthenticationResponse = client
        .post(request_url)
        .json(&payload)
        .header(COOKIE, cookie_string)
        .send()
        .await?
        .json()
        .await?;
    let token = access_token_response.Token;
    let user_hash = &access_token_response.DisplayClaims.xui[0].uhs;
    let not_after = &access_token_response.NotAfter;
    return Ok(AuthenticationResult {
        token,
        user_hash: user_hash.to_owned(),
        not_after: not_after.to_owned(),
        cookies,
    });
}
