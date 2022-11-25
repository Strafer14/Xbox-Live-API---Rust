use async_std;
mod app;

#[async_std::main]
async fn main() {
    let result = app::api::fetch_profile("Ninja").await;
    print!("{:?}", result.unwrap());
}
