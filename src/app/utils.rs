use std::env;

use reqwest::{
    header::{AUTHORIZATION, CONTENT_TYPE, COOKIE, USER_AGENT},
    RequestBuilder,
};

pub fn add_headers(request: RequestBuilder, cookie: String, authorization_header: String) -> RequestBuilder {
    return request
    .header(COOKIE, cookie)
    .header(CONTENT_TYPE, r#"application/json"#)
    .header("x-xbl-contract-version", "2")
    .header(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/60.0.3112.113 Safari/537.36 Like SmartGlass/2.105.0415 CFNetwork/711.3.18 Darwin/14.0.0")
    .header(AUTHORIZATION, authorization_header);
}
