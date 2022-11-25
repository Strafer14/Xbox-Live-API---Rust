use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, COOKIE, USER_AGENT};
use reqwest::Result;
use std::env;
use urlencoding::encode;

use super::types::{
    GetAchievementsReponse, GetClipsResponse, GetScreenshotsResponse, Profile, ProfileResponse, GetActivityResponse,
};

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

pub async fn fetch_screenshots(xuid: &str) -> Result<GetScreenshotsResponse> {
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
    let screenshots_response = response.json::<GetScreenshotsResponse>().await?;
    return Ok(screenshots_response);
}

pub async fn fetch_achievements(xuid: &str) -> Result<GetAchievementsReponse> {
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
    let achievements_response = response.json::<GetAchievementsReponse>().await?;
    return Ok(achievements_response);
}

pub async fn fetch_activities(xuid: &str) -> Result<GetActivityResponse> {
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
    let activity_response = response.json::<GetActivityResponse>().await?;
    return Ok(activity_response);
}
