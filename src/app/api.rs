use std::env;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, COOKIE, USER_AGENT};
use reqwest::Result;
use serde::Deserialize;
use urlencoding::encode;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ProfileResponse {
    profile_users: Vec<Profile>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    id: String,
    host_id: String,
    is_sponsored_user: bool,
}

pub async fn fetch_profile(gamer_tag: &str) -> Result<Profile> {
    let request_url = format!(
        "https://profile.xboxlive.com/users/gt({gamertag})/profile/settings",
        gamertag = encode(gamer_tag),
    );
    let client = reqwest::Client::new();
    let response = client
        .get(&request_url)
        .header(COOKIE, env::var("COOKIES").unwrap())
        .header(CONTENT_TYPE, "application/json")
        .header("x-xbl-contract-version", "2")
        .header(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/60.0.3112.113 Safari/537.36 Like SmartGlass/2.105.0415 CFNetwork/711.3.18 Darwin/14.0.0")
        .header(AUTHORIZATION, env::var("AUTHORIZATION").unwrap())
        .send()
        .await?;
    let profile = response.json::<ProfileResponse>().await?;
    let profile_users = profile.profile_users;
    let found_user = profile_users.first().cloned().unwrap();
    return Ok(found_user);
}
