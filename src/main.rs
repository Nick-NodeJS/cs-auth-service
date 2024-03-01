pub mod app;
pub mod config;
pub mod tests;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    if let Err(err) = app::run().await {
        println!("Error to start Auth Service: {:?}", err)
    }
    Ok(())
}

/*
TODO:
- investigate if it makes sense to enable caching on handlers
- implement me endpoint
- set swagger
- add tests
Extra:
- investigate Twitter, GitHub etc
- investigate if it makes sense to add Postgres as data storage
 */
