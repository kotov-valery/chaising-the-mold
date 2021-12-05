
#[get("/")]
pub async fn hello() -> &'static str {
    "Hello, world!"
}