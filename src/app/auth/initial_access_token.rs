use super::{pre_auth::FetchPreAuthDataResponse, utils::{generate_post_values, extract_access_token, turn_cookie_vector_to_string}};
use reqwest::{
    header::{CONTENT_TYPE, COOKIE},
    Url,
};

const HOST: &str = "login.live.com";

#[derive(Debug)]
pub struct FetchInitialTokenResultData {
    // Vector of cookie key, values
    pub cookies: Vec<(String, String)>,
    pub access_token: String,
}

pub async fn fetch_initial_access_token(
    pre_auth_response: reqwest::Result<FetchPreAuthDataResponse>,
) -> reqwest::Result<FetchInitialTokenResultData> {
    let pre_auth_response_unwrapped = pre_auth_response.unwrap();
    let cookies = pre_auth_response_unwrapped.cookies;
    let url_post_and_ppft_re = pre_auth_response_unwrapped.url_post_and_ppft_re;
    let url_post = &url_post_and_ppft_re[0].1;
    let ppft_re = &url_post_and_ppft_re[1].1;
    let post_values = generate_post_values(ppft_re.to_string());
    let url = Url::parse(&url_post).unwrap();
    let path = url.path().to_owned() + "?" + url.query().unwrap();
    if path.is_empty() {
        panic!("No path found on query params");
    }

    let client = reqwest::Client::new();
    let cookie_string = turn_cookie_vector_to_string(&cookies);

    let request_url = "https://".to_owned() + HOST + &path;
    let access_token_response = client
        .post(request_url)
        .body(post_values)
        .header(COOKIE, cookie_string)
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .send()
        .await?;
    let status = &access_token_response.status().as_u16();
    if [302, 200].contains(status) {
        let url = access_token_response.url().as_str();
        let access_token = extract_access_token(url.to_string());
        if access_token.is_empty() {
            panic!("Could not find access token");
        }
        Ok(FetchInitialTokenResultData {
            cookies,
            access_token: access_token,
        })
    } else {
        panic!("Could not get access token");
    }
}
