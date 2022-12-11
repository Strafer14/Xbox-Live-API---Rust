use reqwest::Result;
use urlencoding::encode;

use super::auth::flow::fetch_cookies_and_authorization;
use super::types::{
    GetAchievementsReponse, GetActivityResponse, GetClipsResponse, GetScreenshotsResponse, Profile,
    ProfileResponse,
};
use super::utils::add_headers;

pub async fn fetch_profile(gamer_tag: &str) -> Result<Profile> {
    let encoded_gamer_tag = encode(gamer_tag);
    let request_url = format!(
        "https://profile.xboxlive.com/users/gt({gamertag})/profile/settings",
        gamertag = encoded_gamer_tag,
    );
    let client = reqwest::Client::new();
    let auth_data = fetch_cookies_and_authorization().await?;
    let response = add_headers(
        client.get(&request_url),
        auth_data.cookie,
        auth_data.authorization_header,
    )
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
    let auth_data = fetch_cookies_and_authorization().await?;
    let response = add_headers(
        client.get(&request_url),
        auth_data.cookie,
        auth_data.authorization_header,
    )
    .send()
    .await?;
    let clips_response = response.json::<GetClipsResponse>().await?;
    return Ok(clips_response);
}

pub async fn fetch_screenshots(xuid: &str) -> Result<GetScreenshotsResponse> {
    let request_url = format!(
        "https://screenshotsmetadata.xboxlive.com/users/xuid({xuid})/screenshots",
        xuid = xuid,
    );
    let client = reqwest::Client::new();
    let auth_data = fetch_cookies_and_authorization().await?;
    let response = add_headers(
        client.get(&request_url),
        auth_data.cookie,
        auth_data.authorization_header,
    )
    .send()
    .await?;
    let screenshots_response = response.json::<GetScreenshotsResponse>().await?;
    return Ok(screenshots_response);
}

pub async fn fetch_achievements(xuid: &str) -> Result<GetAchievementsReponse> {
    let request_url = format!(
        "https://achievements.xboxlive.com/users/xuid({xuid})/history/titles",
        xuid = xuid,
    );
    let client = reqwest::Client::new();
    let auth_data = fetch_cookies_and_authorization().await?;
    let response = add_headers(
        client.get(&request_url),
        auth_data.cookie,
        auth_data.authorization_header,
    )
    .send()
    .await?;
    let achievements_response = response.json::<GetAchievementsReponse>().await?;
    return Ok(achievements_response);
}

pub async fn fetch_activities(xuid: &str) -> Result<GetActivityResponse> {
    let request_url = format!(
        "https://avty.xboxlive.com/users/xuid({xuid})/activity/history",
        xuid = xuid,
    );
    let client = reqwest::Client::new();
    let auth_data = fetch_cookies_and_authorization().await?;
    let response = add_headers(
        client.get(&request_url),
        auth_data.cookie,
        auth_data.authorization_header,
    )
    .send()
    .await?;
    let activity_response = response.json::<GetActivityResponse>().await?;
    return Ok(activity_response);
}
