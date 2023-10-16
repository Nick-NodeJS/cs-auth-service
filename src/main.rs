// import the app module
mod app;
mod config;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    if let Err(err) = app::run().await {
        println!("Error to start Auth Service: {:?}", err)
    }
    Ok(())
}
