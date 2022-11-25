use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, COOKIE, USER_AGENT};
use reqwest::Result;
use serde::Deserialize;
use std::env;
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

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PagingInfo {
    continuation_token: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Clip {
    game_clip_id: String,
    state: String,
    date_published: String,
    date_recorded: String,
    last_modified: String,
    user_caption: String,
    r#type: String,
    duration_in_seconds: i32,
    scid: String,
    title_id: i32,
    rating: i32,
    rating_count: i32,
    views: i32,
    title_data: String,
    system_properties: String,
    saved_by_user: bool,
    achievement_id: String,
    greatest_moment_id: String,
    // thumbnails: array<{
    //   uri: String,
    //   file_size: i32,
    //   thumbnail_type: String,
    // }>;
    // game_clip_uris: array<{
    //   uri: String,
    //   file_size: i32,
    //   uri_type: String,
    //   expiration: String,
    // }>;
    xuid: String,
    clip_name: String,
    title_name: String,
    game_clip_locale: String,
    clip_content_attributes: String,
    device_type: String,
    comment_count: i32,
    like_count: i32,
    share_count: i32,
    partial_views: i32,
  }
  #[derive(Clone, Debug, Deserialize)]
  #[serde(rename_all = "camelCase")]
  struct GetClipsResponse {
    game_clips: Vec<Clip>,
    paging_info: PagingInfo
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

pub async fn fetch_clips(xuid: &str) -> Result<GetClipsResponse> {
    let request_url = format!(
        "https://gameclipsmetadata.xboxlive.com/users/xuid({xuid})/clips",
        xuid = xuid,
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
    let clips_response = response.json::<GetClipsResponse>().await?;
    return Ok(clips_response);
}

pub async fn fetch_screenshots(xuid: &str) -> Result<Profile> {
    let request_url = format!(
        "https://screenshotsmetadata.xboxlive.com/users/xuid({xuid})/screenshots",
        xuid = xuid,
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

pub async fn fetch_achievements(xuid: &str) -> Result<Profile> {
    let request_url = format!(
        "https://achievements.xboxlive.com/users/xuid({xuid})/history/titles",
        xuid = xuid,
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

pub async fn fetch_activities(xuid: &str) -> Result<Profile> {
    let request_url = format!(
        "https://avty.xboxlive.com/users/xuid({xuid})/activity/history",
        xuid = xuid,
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
