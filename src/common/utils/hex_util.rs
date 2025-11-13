pub fn trim_hex_prefix(input: &str) -> &str {
    if input.starts_with("0x") || input.starts_with("0X") {
        &input[2..]
    } else {
        input
    }
}

pub fn is_valid_hex(s: &str) -> bool {
    let s = trim_hex_prefix(s);
    s.len() % 2 == 0 && s.chars().all(|c| c.is_ascii_hexdigit())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trim_hex_prefix() {
        assert_eq!(trim_hex_prefix("0x1234"), "1234");
        assert_eq!(trim_hex_prefix("0X1234"), "1234");
        assert_eq!(trim_hex_prefix("1234"), "1234");
    }

    #[test]
    fn test_is_valid_hex() {
        assert_eq!(is_valid_hex("0x1234"), true);
        assert_eq!(is_valid_hex("0X1234"), true);
        assert_eq!(is_valid_hex("1234"), true);
        assert_eq!(is_valid_hex("0y1234"), false);
        assert_eq!(is_valid_hex("1234x"), false);
    }

}