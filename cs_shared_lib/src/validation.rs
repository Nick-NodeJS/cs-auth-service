use std::{net::Ipv4Addr, str::FromStr};

pub fn is_valid_ipv4(ip: &str) -> bool {
    match Ipv4Addr::from_str(ip) {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn validate_ip_port(_e: u16) -> bool {
    // No need to compare to 0 or 65535 explicitly.
    // value is always within the range of u16.
    true
}

/// provide integer validation in given range
pub fn validate_integer_in_range<T>(value: T, min: T, max: T) -> bool
where
    T: std::cmp::PartialOrd,
{
    value >= min && value <= max
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
