pub fn is_valid_ipv4(ip: &str) -> bool {
    // TODO upgrade this Regex validation to use some lib

    let ip_pattern = regex::Regex::new(r"^(?:[0-9]{1,3}\.){3}[0-9]{1,3}$").unwrap();
    ip_pattern.is_match(ip)
}

pub fn validate_ip_port(_e: u16) -> bool {
    // No need to compare to 0 or 65535 explicitly.
    // value is always within the range of u16.
    true
}

pub fn validate_integer_in_range<T>(value: T, min: T, max: T) -> bool
where
    T: std::cmp::PartialOrd,
{
    value >= min && value <= max
}

// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
