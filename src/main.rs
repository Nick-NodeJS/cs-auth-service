// import the app module
mod app;
mod config;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    app::run().await
}
