use jsonwebtoken::{
    decode, encode, errors::Error, Algorithm, DecodingKey, EncodingKey, Header, TokenData,
    Validation,
};
use serde::{de::DeserializeOwned, Serialize};

pub fn decode_token<T>(token: &str, key: &DecodingKey, check_expiration: bool) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    // Validation configuration
    let mut validation = Validation::new(Algorithm::RS256);
    if !check_expiration {
        validation.validate_exp = false;
    }

    let token_data: TokenData<T> = match decode(token, key, &validation) {
        Ok(data) => data,
        Err(err) => {
            log::warn!("Decode Error: {}\n token: {}\n", err, token);
            return Err(err);
        }
    };

    Ok(token_data.claims)
}

pub fn decoding_key_from_cert(modulus: &str, exponent: &str) -> Result<DecodingKey, Error> {
    let key = DecodingKey::from_rsa_components(modulus, exponent)?;
    Ok(key)
}

pub fn encode_claims<T>(claims: &T, secret: &str, header: Option<&Header>) -> Result<String, Error>
where
    T: Serialize,
{
    let default_header = Header::default();
    let own_header = match header {
        Some(h) => h,
        None => &default_header,
    };
    let token = encode(
        own_header,
        claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )?;
    Ok(token)
}
