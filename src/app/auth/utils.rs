use std::collections::HashMap;

use regex::Regex;

pub fn extract_url_post_and_ppft_re(payload: String) -> [(String, String); 2] {
    let re_url_post = Regex::new(r"urlPost:'([A-Za-z0-9:\?_\-\.&\\/=]+)").unwrap();
    let url_post = re_url_post.captures(&payload).unwrap();
    let re_ppft_re = Regex::new(r#"sFTTag:'.*value="(.*)"/>'"#).unwrap();
    let re_ppft = re_ppft_re.captures(&payload).unwrap();

    return [
        (
            "url_post".to_string(),
            url_post.get(1).unwrap().as_str().to_string(),
        ),
        (
            "re_ppft".to_string(),
            re_ppft.get(1).unwrap().as_str().to_string(),
        ),
    ];
}

pub fn generate_post_values(ppft_re: String) -> String {
    let mut post_values = HashMap::new();
    post_values.insert(
        "login".to_string(),
        std::env::var("XBL_USERNAME").unwrap_or_default(),
    );
    post_values.insert(
        "passwd".to_string(),
        std::env::var("XBL_PASSWORD").unwrap_or_default(),
    );
    post_values.insert("PPFT".to_string(), ppft_re);
    post_values.insert("PPSX".to_string(), "Passpor".to_string());
    post_values.insert("SI".to_string(), "Sign In".to_string());
    post_values.insert("type".to_string(), "11".to_string());
    post_values.insert("NewUser".to_string(), "1".to_string());
    post_values.insert("LoginOptions".to_string(), "1".to_string());
    post_values.insert("i3".to_string(), "36728".to_string());
    post_values.insert("m1".to_string(), "768".to_string());
    post_values.insert("m2".to_string(), "1184".to_string());
    post_values.insert("m3".to_string(), "0".to_string());
    post_values.insert("i12".to_string(), "1".to_string());
    post_values.insert("i17".to_string(), "0".to_string());
    post_values.insert("i18".to_string(), "__Login_Host|1".to_string());
    let query_string = serde_qs::to_string(&post_values).unwrap();
    return query_string;
}


pub fn extract_access_token(url: String) -> String {
    let access_token_re = Regex::new(r"access_token=(.+?)&").unwrap();
    let access_token = access_token_re.captures(&url).unwrap();
    return access_token.get(1).unwrap().as_str().to_string();
}