use reqwest::header::HOST;
use reqwest::{Result};

use super::utils::extract_url_post_and_ppft_re;

pub struct FetchPreAuthDataResponse {
    // Vector of cookie key, values
    pub cookies: Vec<(String, String)>,
    pub url_post_and_ppft_re: [(String, String); 2],
}

pub async fn fetch_pre_auth_data() -> Result<FetchPreAuthDataResponse> {
    let host_name = "login.live.com";
    let client = reqwest::Client::new();
    let query = [
        ("client_id", "0000000048093EE3"),
        ("redirect_uri", "https://login.live.com/oauth20_desktop.srf"),
        ("response_type", "token"),
        ("display", "touch"),
        ("scope", "service::user.auth.xboxlive.com::MBI_SSL"),
        ("locale", "en"),
    ];
    let request_url = format!(
        "https://{host_name}/oauth20_authorize.srf",
        host_name = host_name
    );
    let response = client
        .get(&request_url)
        .header(HOST, host_name)
        .query(&query)
        .send()
        .await?;
    let cookies = response.cookies();
    let mut cookie_kvs: Vec<(String, String)> = Vec::new();
    for cookie in cookies {
        cookie_kvs.push((cookie.name().to_string(), cookie.value().to_string()));
    }

    let content = response.text().await?;
    let url_post_and_ppft_re = extract_url_post_and_ppft_re(content);
    let result = FetchPreAuthDataResponse {
        cookies: cookie_kvs,
        url_post_and_ppft_re: url_post_and_ppft_re,
    };
    return Ok(result);
}
