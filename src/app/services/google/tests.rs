#[cfg(test)]
mod tests {
    use jsonwebtoken::DecodingKey;

    use crate::{
        app::services::google::service::decode_token, config::google_config::GoogleConfig,
    };

    #[test]
    fn test_google_token_decoding() {
        let google_config = GoogleConfig::new();
        let token: &str = &google_config.google_test_token;

        let key = DecodingKey::from_rsa_components(
            "uB-3s136B_Vcme1zGQEg-Avs31_voau8BPKtvbYhB0QOHTtrXCF_wxIH5vWjl-5ts8up8Iy2kVnaItsecGohBAy_0kRgq8oi-n_cZ0i5bspAX5VW0peh_QU3KTlKSBaz3ZD9xMCDWuJFFniHuxLtJ4QtL4v2oDD3pBPNRPyIcZ_LKhH3-Jm-EAvubI5-6lB01zkP5x8f2mp2upqAmyex0jKFka2e0DOBavmGsGvKHKtTnE9oSOTDlhINgQPohoSmir89NRbEqqzeZVb55LWRl_hkiDDOZmcM_oJ8iUbm6vQu3YwCy-ef9wGYEij5GOWLmpYsws5vLVtTE2U-0C_ItQ",
            "AQAB"
        ).expect("Error to make decoding key");

        // need to disable expiration checking if token outdated
        // locally you can set fresh token and enable it
        match decode_token(token, &key, false) {
            Ok(token_data) => {
                println!("Decoded token data: {:?}\n {}", token_data, token);
                assert_eq!(token_data.aud, google_config.google_client_id);
            }
            Err(err) => {
                debug_assert!(false, "Error: {}", err)
            }
        };
    }
}