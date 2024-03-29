#[cfg(test)]
mod tests {
    use env_logger::Env;
    use jsonwebtoken::DecodingKey;

    use crate::app::{
        providers::google::common::TokenClaims, shared::jwt::decode_token as decode_google_token,
    };

    #[test]
    /*
        TODO:
       It must to make GAPI request to get fresh key. Response has couple of options.
       The right option is the one which 'kid' is inside of token header
    */
    fn decode_token() {
        // Set logger for all tests
        env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

        let token = dotenv::var("GOOGLE_TEST_TOKEN").expect("Bad GOOGLE_TEST_TOKEN env");
        let google_client_id = dotenv::var("GOOGLE_CLIENT_ID")
            .expect("Missing the GOOGLE_CLIENT_ID environment variable.");

        // The key should be taken from GAPI when token above was issued to be able to decode it
        let key = DecodingKey::from_rsa_components(
            "uB-3s136B_Vcme1zGQEg-Avs31_voau8BPKtvbYhB0QOHTtrXCF_wxIH5vWjl-5ts8up8Iy2kVnaItsecGohBAy_0kRgq8oi-n_cZ0i5bspAX5VW0peh_QU3KTlKSBaz3ZD9xMCDWuJFFniHuxLtJ4QtL4v2oDD3pBPNRPyIcZ_LKhH3-Jm-EAvubI5-6lB01zkP5x8f2mp2upqAmyex0jKFka2e0DOBavmGsGvKHKtTnE9oSOTDlhINgQPohoSmir89NRbEqqzeZVb55LWRl_hkiDDOZmcM_oJ8iUbm6vQu3YwCy-ef9wGYEij5GOWLmpYsws5vLVtTE2U-0C_ItQ",
            "AQAB"
        ).expect("Error to make decoding key");

        // need to disable expiration checking if token outdated
        // locally you can set fresh token + key and enable it
        match decode_google_token::<TokenClaims>(&token, &key, false) {
            Ok(token_data) => {
                println!("Decoded token data: {:?}\n {}", token_data, token);
                assert_eq!(token_data.aud, google_client_id);
            }
            Err(err) => {
                debug_assert!(false, "Error: {}", err)
            }
        };
    }
}
