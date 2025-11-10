use hex;
use base64;
use bs58;

pub fn handle_hex_to_bs64(input: &str) -> Option<String> {
    let input = if input.starts_with("0x") {
        &input[2..]
    } else {
        input
    };

    match hex::decode(input) {
        Ok(bytes) => {
            let bs64 = base64::encode(&bytes);
            println!("Base64: {}", bs64);
            Some(bs64)
        },
        Err(e) => {
            println!("Error decoding hex string: {}", e);
            None
        }
    }
}

pub fn handle_hex_to_bs58(input: &str) -> Option<String> {
    let input = if input.starts_with("0x") {
        &input[2..]
    } else {
        input
    };

    match hex::decode(input) {
        Ok(bytes) => {
            let bs58_str = bs58::encode(&bytes).into_string();
            println!("Base58: {}", bs58_str);
            Some(bs58_str)
        },
        Err(e) => {
            println!("Error decoding hex string: {}", e);
            None
        }
    }
}

pub fn handle_bs64_to_hex(input: &str) -> Option<String> {
    match base64::decode(input) {
        Ok(bytes) => {
            let hex_str = hex::encode(&bytes);
            println!("Hex: {}", hex_str);
            Some(hex_str)
        },
        Err(e) => {
            println!("Error decoding base64 string: {}", e);
            None
        }
    }
}

pub fn handle_bs64_to_bs58(input: &str) -> Option<String> {
    match base64::decode(input) {
        Ok(bytes) => {
            let bs58_str = bs58::encode(&bytes).into_string();
            println!("Base58: {}", bs58_str);
            Some(bs58_str)
        },
        Err(e) => {
            println!("Error decoding base64 string: {}", e);
            None
        }
    }
}

pub fn handle_bs58_to_hex(input: &str) -> Option<String> {
    match bs58::decode(input).into_vec() {
        Ok(bytes) => {
            let hex_str = hex::encode(&bytes);
            println!("Hex: {}", hex_str);
            Some(hex_str)
        },
        Err(e) => {
            println!("Error decoding base58 string: {}", e);
            None
        }
    }
}

pub fn handle_bs58_to_bs64(input: &str) -> Option<String> {
    match bs58::decode(input).into_vec() {
        Ok(bytes) => {
            let bs64_str = base64::encode(&bytes);
            println!("Base64: {}", bs64_str);
            Some(bs64_str)
        },
        Err(e) => {
            println!("Error decoding base58 string: {}", e);
            None
        }
    }
}

pub fn handle_bytes_to_hex(bytes_str: &str) -> Option<String> {
    let bytes = bytes_str_to_bytes(bytes_str);
    if bytes.is_none() {
        println!("Invalid bytes string");
        return None;
    }
    let bytes = bytes.unwrap();
    let hex_str = hex::encode(&bytes);
    println!("Hex: {}", hex_str);
    Some(hex_str)
}

pub fn bytes_str_to_bytes(bytes_str: &str) -> Option<Vec<u8>> {
    // [1,2,3,255]
    let bytes_str = bytes_str.trim();
    if !bytes_str.starts_with("[") || !bytes_str.ends_with("]") {
        return None;
    }

    let inner = &bytes_str[1..bytes_str.len()-1];

    if inner.trim().is_empty() {
        return Some(vec![]);
    }

    inner.split(',')
        .map(|s| s.trim().parse::<u8>())
        .collect::<Result<Vec<u8>, _>>()
        .ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_base64() {
        assert_eq!(handle_hex_to_bs64("00").unwrap(), "AA==");
        assert_eq!(handle_hex_to_bs64("0x01").unwrap(), "AQ==");
        assert_eq!(handle_hex_to_bs64("0"), None);
    }

    #[test]
    fn test_hex_to_base58() {
        assert_eq!(handle_hex_to_bs58("0000").unwrap(), "11");
        assert_eq!(handle_hex_to_bs58("0x02").unwrap(), "3");
        assert_eq!(handle_hex_to_bs58("0"), None);
    }

    #[test]
    fn test_bs64_to_hex() {
        assert_eq!(handle_bs64_to_hex("AA==").unwrap(), "00");
        assert_eq!(handle_bs64_to_hex("AQ==").unwrap(), "01");
        assert_eq!(handle_bs64_to_hex("0"), None);
    }

    #[test]
    fn test_handle_bs64_to_bs58() {
        assert_eq!(handle_bs64_to_bs58("AA==").unwrap(), "1");
        assert_eq!(handle_bs64_to_bs58("AQ==").unwrap(), "2");
        assert_eq!(handle_bs64_to_bs58("0"), None);
    }

    #[test]
    fn test_handle_bs58_to_hex() {
        assert_eq!(handle_bs58_to_hex("11").unwrap(), "0000");
        assert_eq!(handle_bs58_to_hex("3").unwrap(), "02");
        assert_eq!(handle_bs58_to_hex("0"), None);
    }

    #[test]
    fn test_handle_bs58_to_bs64() {
        assert_eq!(handle_bs58_to_bs64("1").unwrap(), "AA==");
        assert_eq!(handle_bs58_to_bs64("2").unwrap(), "AQ==");
        assert_eq!(handle_bs58_to_bs64("0"), None);
    }

    #[test]
    fn test_bytes_str_to_bytes() {
        assert_eq!(bytes_str_to_bytes("[1,2,3,255]").unwrap(), vec![1,2,3,255]);
        assert_eq!(bytes_str_to_bytes("[1,2"), None);
        assert_eq!(bytes_str_to_bytes("1"), None);
    }

    #[test]
    fn test_handle_bytes_to_hex() {
        assert_eq!(handle_bytes_to_hex("[1,2,3,255]").unwrap(), "010203ff");
        assert_eq!(handle_bytes_to_hex(" [ 1  , 2,3,255 ]  ").unwrap(), "010203ff");
        assert_eq!(handle_bytes_to_hex("[1,2"), None);
        assert_eq!(handle_bytes_to_hex("1"), None);
    }
}
