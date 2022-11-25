use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileResponse {
    pub profile_users: Vec<Profile>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub id: String,
    pub host_id: String,
    // settings: Vec<unknown>,
    pub is_sponsored_user: bool,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PagingInfo {
    continuation_token: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Clip {
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
pub struct GetClipsResponse {
    game_clips: Vec<Clip>,
    paging_info: PagingInfo,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScreenShot {
    screenshot_id: String,
    resolution_height: i32,
    resolution_width: i32,
    state: String,
    date_published: String,
    date_taken: String,
    last_modified: String,
    user_caption: String,
    r#type: String,
    scid: String,
    title_id: i32,
    rating: i32,
    rating_count: i32,
    views: i32,
    title_data: String,
    system_properties: String,
    saved_by_user: bool,
    achievement_id: String,
    greatest_moment_id: Option<String>,
    // thumbnails: array<{
    //   uri: String,
    //   file_size: i32,
    //   thumbnail_type: String,
    // }>;
    // screenshot_uris: array<{
    //   uri: String,
    //   file_size: i32,
    //   uri_type: String,
    //   expiration: String,
    // }>;
    xuid: String,
    screenshot_name: String,
    title_name: String,
    screenshot_locale: String,
    screenshot_content_attributes: String,
    device_type: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetScreenshotsResponse {
    screenshots: Vec<ScreenShot>,
    paging_info: PagingInfo,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Title {
    last_unlock: String,
    title_id: i32,
    service_config_id: String,
    title_type: String,
    platform: String,
    name: String,
    earned_achievements: i32,
    current_gamerscore: i32,
    max_gamerscore: i32,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAchievementsReponse {
    titles: Vec<Title>,
    paging_info: PagingInfo,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityItem {
    achievement_scid: String,
    achievement_id: String,
    achievement_type: String,
    achievement_icon: String,
    gamerscore: i32,
    achievement_name: String,
    achievement_description: String,
    is_secret: bool,
    has_app_award: bool,
    has_art_award: bool,
    content_image_uri: String,
    content_title: String,
    platform: String,
    title_id: String,
    // activity: {
    //   num_shares: i32,
    //   num_likes: i32,
    //   num_comments: i32,
    //   ugc_caption: nullable<unknown>;
    //   achievement_scid: String,
    //   achievement_id: String,
    //   activity_item_type: String,
    //   achievement_type: String,
    //   user_xuid: String,
    //   achievement_icon: String,
    //   author_type: String,
    //   gamerscore: i32,
    //   date: String,
    //   achievement_name: String,
    //   content_type: String,
    //   achievement_description: String,
    //   title_id: String,
    //   is_secret: bool,
    //   platform: String,
    //   shared_source_user: i32,
    //   sandboxid: String,
    //   rarity_category: String,
    //   user_key: nullable<unknown>;
    //   rarity_percentage: i32,
    //   scid: String,
    //   date_override: String,
    //   is_mock: bool,
    //   is_user_post: bool,
    //   locator: String,
    // };
    user_image_uri_md: String,
    user_image_uri_xs: String,
    description: String,
    date: String,
    has_ugc: bool,
    activity_item_type: String,
    content_type: String,
    short_description: String,
    item_text: String,
    item_image: String,
    share_root: String,
    feed_item_id: String,
    item_root: String,
    has_liked: bool,
    // author_info: {
    //   name: String,
    //   second_name: String,
    //   image_url: String,
    //   author_type: String,
    //   id: String,
    // };
    gamertag: String,
    real_name: String,
    display_name: String,
    user_image_uri: String,
    user_xuid: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetActivityResponse {
    activity_items: Vec<ActivityItem>,
    cont_token: String,
    num_items: i32,
    polling_interval_seconds: String,
    polling_token: String,
}
