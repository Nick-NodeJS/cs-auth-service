use jsonwebtoken::{DecodingKey, Validation, Algorithm, decode};
use serde::Deserialize;

// import the app module
mod app;
mod config;

// #[derive(Debug, Deserialize)]
// struct Claims {
//     sub: String,  // This should match one of the claims in the token
//     // Add other claim fields here that you expect in the token
// }

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // fn main() -> () {
    if let Err(err) = app::run().await {
        println!("Error to start Auth Service: {:?}", err)
    }
    Ok(())

    // The token, key, and validation configuration
    // let token = "ya29.a0AfB_byByPqYWjgxasfvxtuPFKvnuu6uS9Y7Tw5UIWC6kCMpt25BPTmYeqcQxI9IK_iMaOV-TfebaEfWpAQL1AL83P-SWcjYp-dMr082FwNYD4wuYv8jLVH77OKFJ-5VL2h-iK978TMjY56qDZYUVuZCivLVFg_jYuPS5aCgYKATkSARESFQGOcNnCXRzrsSi4TibZ0lyGTuxHyg0171";  // Replace with your actual access token

    // let decoding_key = DecodingKey::from_rsa_components(
    //     "q5hcowR4IuPiSvHbwj9Rv9j2XRnrgbAAFYBqoLBwUV5GVIiNPKnQBYa8ZEIK2naj9gqpo3DU9lx7d7RzeVlzCS5eUA2LV94--KbT0YgIJnApj5-hyDIaevI1Sf2YQr_cntgVLvxqfW1n9ZvbQSitz5Tgh0cplZvuiWMFPu4_mh6B3ShEKIl-qi-h0cZJlRcIf0ZwkfcDOTE8bqEzWUvlCpCH9FK6Mo9YLjw5LroBcHdUbOg3Keu0uW5SCEi-2XBQgCF6xF3kliciwwnv2HhCPyTiX0paM_sT2uKspYock-IQglQ2TExoJqbYZe6CInSHiAA68fkSkJQDnuRZE7XTJQ",
    //     "AQAB",
    // ).expect("error to create decoding key");
    // let validation = Validation::new(Algorithm::RS256);

    // match decode::<Claims>(token, &decoding_key, &validation) {
    //     Ok(token_data) => {
    //         println!("Token is valid: {:?}", token_data.claims);
    //     }
    //     Err(err) => {
    //         println!("Token decoding error: {:?}", err);
    //     }
    // }
}
