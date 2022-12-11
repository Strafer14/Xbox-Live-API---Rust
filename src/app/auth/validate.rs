use std::time::{SystemTime, UNIX_EPOCH};

pub const ONE_HOUR: u64 = 3600;

pub fn validate_token_still_valid(not_after_in_seconds: String) -> bool {
    let not_after_in_seconds_numeric =  not_after_in_seconds.parse::<u64>().unwrap();
    let time_now = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_secs();
    return not_after_in_seconds_numeric - time_now < ONE_HOUR;
}