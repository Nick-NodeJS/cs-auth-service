mod app;
mod config;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // fn main() -> () {
    if let Err(err) = app::run().await {
        println!("Error to start Auth Service: {:?}", err)
    }
    Ok(())
}

/*
TODO:
- finish sessions
- implement caching on user/storage services
- investigate if it makes sense to enable caching on handlers
- implement me endpoint
- set swagger
- add tests
Extra:
- add Facebook auth provider
- investigate Twitter, GitHub etc
- investigate if it makes sense to add Postgres as data storage
 */
