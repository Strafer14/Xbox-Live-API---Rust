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
pub struct PagingInfo {
    pub continuation_token: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Thumbnail {
    pub uri: String,
    pub file_size: i32,
    pub thumbnail_type: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceUri {
    pub uri: String,
    pub file_size: i32,
    pub uri_type: String,
    pub expiration: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Clip {
    pub game_clip_id: String,
    pub state: String,
    pub date_published: String,
    pub date_recorded: String,
    pub last_modified: String,
    pub user_caption: String,
    pub r#type: String,
    pub duration_in_seconds: i32,
    pub scid: String,
    pub title_id: i32,
    pub rating: i32,
    pub rating_count: i32,
    pub views: i32,
    pub title_data: String,
    pub system_properties: String,
    pub saved_by_user: bool,
    pub achievement_id: String,
    pub greatest_moment_id: String,
    pub thumbnails: Vec<Thumbnail>,
    pub game_clip_uris: Vec<ResourceUri>,
    pub xuid: String,
    pub clip_name: String,
    pub title_name: String,
    pub game_clip_locale: String,
    pub clip_content_attributes: String,
    pub device_type: String,
    pub comment_count: i32,
    pub like_count: i32,
    pub share_count: i32,
    pub partial_views: i32,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetClipsResponse {
    pub game_clips: Vec<Clip>,
    pub paging_info: PagingInfo,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScreenShot {
    pub screenshot_id: String,
    pub resolution_height: i32,
    pub resolution_width: i32,
    pub state: String,
    pub date_published: String,
    pub date_taken: String,
    pub last_modified: String,
    pub user_caption: String,
    pub r#type: String,
    pub scid: String,
    pub title_id: i32,
    pub rating: i32,
    pub rating_count: i32,
    pub views: i32,
    pub title_data: String,
    pub system_properties: String,
    pub saved_by_user: bool,
    pub achievement_id: String,
    pub greatest_moment_id: Option<String>,
    pub thumbnails: Vec<Thumbnail>,
    pub game_clip_uris: Vec<ResourceUri>,
    pub xuid: String,
    pub screenshot_name: String,
    pub title_name: String,
    pub screenshot_locale: String,
    pub screenshot_content_attributes: String,
    pub device_type: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetScreenshotsResponse {
    pub screenshots: Vec<ScreenShot>,
    pub paging_info: PagingInfo,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Title {
    pub last_unlock: String,
    pub title_id: i32,
    pub service_config_id: String,
    pub title_type: String,
    pub platform: String,
    pub name: String,
    pub earned_achievements: i32,
    pub current_gamerscore: i32,
    pub max_gamerscore: i32,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAchievementsReponse {
    pub titles: Vec<Title>,
    pub paging_info: PagingInfo,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Activity {
    pub num_shares: i32,
    pub num_likes: i32,
    pub num_comments: i32,
    pub ugc_caption: Option<String>,
    pub achievement_scid: String,
    pub achievement_id: String,
    pub activity_item_type: String,
    pub achievement_type: String,
    pub user_xuid: String,
    pub achievement_icon: String,
    pub author_type: String,
    pub gamerscore: i32,
    pub date: String,
    pub achievement_name: String,
    pub content_type: String,
    pub achievement_description: String,
    pub title_id: String,
    pub is_secret: bool,
    pub platform: String,
    pub shared_source_user: i32,
    pub sandboxid: String,
    pub rarity_category: String,
    pub user_key: Option<String>,
    pub rarity_percentage: i32,
    pub scid: String,
    pub date_override: String,
    pub is_mock: bool,
    pub is_user_post: bool,
    pub locator: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthorInfo {
    pub name: String,
    pub second_name: String,
    pub image_url: String,
    pub author_type: String,
    pub id: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityItem {
    pub achievement_scid: String,
    pub achievement_id: String,
    pub achievement_type: String,
    pub achievement_icon: String,
    pub gamerscore: i32,
    pub achievement_name: String,
    pub achievement_description: String,
    pub is_secret: bool,
    pub has_app_award: bool,
    pub has_art_award: bool,
    pub content_image_uri: String,
    pub content_title: String,
    pub platform: String,
    pub title_id: String,
    pub activity: Activity,
    pub user_image_uri_md: String,
    pub user_image_uri_xs: String,
    pub description: String,
    pub date: String,
    pub has_ugc: bool,
    pub activity_item_type: String,
    pub content_type: String,
    pub short_description: String,
    pub item_text: String,
    pub item_image: String,
    pub share_root: String,
    pub feed_item_id: String,
    pub item_root: String,
    pub has_liked: bool,
    pub author_info: AuthorInfo,
    pub gamertag: String,
    pub real_name: String,
    pub display_name: String,
    pub user_image_uri: String,
    pub user_xuid: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetActivityResponse {
    pub activity_items: Vec<ActivityItem>,
    pub cont_token: String,
    pub num_items: i32,
    pub polling_interval_seconds: String,
    pub polling_token: String,
}
