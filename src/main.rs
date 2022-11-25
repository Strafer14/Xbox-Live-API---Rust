use async_std;
mod app;

#[async_std::main]
async fn main() {
    let result = app::api::fetch_profile("Daaaavie").await;
    print!("{:?}", result.as_ref().unwrap());
    let xuid = result.unwrap().id;
    let clips = app::api::fetch_clips(&xuid).await;
    print!("{:?}", clips.as_ref().unwrap());
    let screenshots = app::api::fetch_screenshots(&xuid).await;
    print!("{:?}", screenshots.as_ref().unwrap());
    let achievements = app::api::fetch_achievements(&xuid).await;
    print!("{:?}", achievements.as_ref().unwrap());
    let activities = app::api::fetch_activities(&xuid).await;
    print!("{:?}", activities.as_ref().unwrap());
}
