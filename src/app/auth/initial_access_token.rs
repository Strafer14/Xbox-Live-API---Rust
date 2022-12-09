use super::{pre_auth::FetchPreAuthDataResponse, utils::generate_post_values};
use reqwest::{
    header::{COOKIE},
    Url,
};

const HOST: &str = "login.live.com";


#[derive(Debug)]
pub struct ResultData {
    // Vector of cookie key, values
    pub cookies: Vec<(String, String)>,
    pub access_token: String,
}

pub async fn fetch_initial_access_token(
    pre_auth_response: reqwest::Result<FetchPreAuthDataResponse>,
) -> reqwest::Result<ResultData> {
    let pre_auth_response_unwrapped = pre_auth_response.unwrap();
    let cookies = pre_auth_response_unwrapped.cookies;
    let url_post_and_ppft_re = pre_auth_response_unwrapped.url_post_and_ppft_re;
    let url_post = &url_post_and_ppft_re[0].1;
    let ppft_re = &url_post_and_ppft_re[1].1;
    let post_values = generate_post_values(ppft_re.to_string());
    let url = Url::parse(&url_post).unwrap();
    let path = url.path();
    if path.is_empty() {
        print!("yay");
        panic!("No path found on query params");
    }

    let client = reqwest::Client::new();
    let cookie_string = cookies
        .iter()
        .map(|(key, value)| format!("{}={}", key, value))
        .collect::<Vec<String>>()
        .join("; ");
    let access_token_response = client
        .post(&format!("https://{}{}", HOST, path))
        .body(post_values)
        .header(COOKIE, cookie_string)
        .send()
        .await?;
    if [302, 200].contains(&access_token_response.status().as_u16()) {
        let url = access_token_response.url();
        print!("{:?}", access_token_response);
        let access_token = url
            .query_pairs()
            .find(|pair| pair.0 == "access_token")
            .unwrap()
            .1
            .to_string();
        if access_token.is_empty() {
            print!("yay2");
            panic!("Could not find access token");
        }
        Ok(ResultData {
            cookies,
            access_token: access_token,
        })
    } else {
        print!("yay3");
        panic!("Could not get access token");
    }
}
