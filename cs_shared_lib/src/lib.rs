pub fn is_valid_ipv4(ip: &str) -> bool {
    // TODO upgrade this Regex validation to use some lib

    let ip_pattern = regex::Regex::new(r"^(?:[0-9]{1,3}\.){3}[0-9]{1,3}$").unwrap();
    ip_pattern.is_match(ip)
}

pub fn validate_ip_port(value: i32) -> bool {
    // TODO: check whether it's correct range
    value >= 1 && value <= 64000
}

pub fn validate_integer_in_range(value: i32, min: i32, max: i32) -> bool {
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
