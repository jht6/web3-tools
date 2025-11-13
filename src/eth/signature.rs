use crate::common::utils::hex_util;
pub fn handle_sig_to_rsv(signature_hex: &str) -> Option<(String, String, u8)> {
    if !validate_signature(signature_hex) {
        println!("Invalid signature");
        return None;
    }
    
    let signature_hex = hex_util::trim_hex_prefix(signature_hex);
    let r = signature_hex[0..64].to_string();
    let s = signature_hex[64..128].to_string();
    let v = if signature_hex.ends_with("1b") {
        27
    } else {
        28
    };

    println!("r: {}", r);
    println!("s: {}", s);
    println!("v: {}", v);


    Some((r, s, v))
}

fn validate_signature(s: &str) -> bool {
    if !hex_util::is_valid_hex(s) {
        return false;
    }
    let s = hex_util::trim_hex_prefix(s);

    // r(32 bytes) + s(32 bytes) + v(1 byte) = 65 bytes = 130 hex chars
    if s.len() != 130 {
        return false;
    }

    // v 的十六进制应该是 1b 或 1c
    if !s.ends_with("1b") && !s.ends_with("1c") {
        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_signature() {
        assert_eq!(validate_signature("0x2c6404e1145f38a8eb7b6a5d7648e6a711f3dfd32e7d7fa8a6c4b17e6b6c6b6d56c4f0a1b0494c6578723e1c5e8f302ff7f6ba674fbec6d1571318c43259b2e41b"), true);
        assert_eq!(validate_signature("0x2c6404e1145f38a8eb7b6a5d7648e6a711f3dfd32e7d7fa8a6c4b17e6b6c6b6d56c4f0a1b0494c6578723e1c5e8f302ff7f6ba674fbec6d1571318c43259b2e41c"), true);
        assert_eq!(validate_signature("2c6404e1145f38a8eb7b6a5d7648e6a711f3dfd32e7d7fa8a6c4b17e6b6c6b6d56c4f0a1b0494c6578723e1c5e8f302ff7f6ba674fbec6d1571318c43259b2e41c"), true);
        assert_eq!(validate_signature("0xxc6404e1145f38a8eb7b6a5d7648e6a711f3dfd32e7d7fa8a6c4b17e6b6c6b6d56c4f0a1b0494c6578723e1c5e8f302ff7f6ba674fbec6d1571318c43259b2e41c"), false);
    }

    #[test]
    fn test_handle_sig_to_rsv() {
        assert_eq!(
            handle_sig_to_rsv("0x2c6404e1145f38a8eb7b6a5d7648e6a711f3dfd32e7d7fa8a6c4b17e6b6c6b6d56c4f0a1b0494c6578723e1c5e8f302ff7f6ba674fbec6d1571318c43259b2e41b"),
            Some((
                "2c6404e1145f38a8eb7b6a5d7648e6a711f3dfd32e7d7fa8a6c4b17e6b6c6b6d".to_string(),
                "56c4f0a1b0494c6578723e1c5e8f302ff7f6ba674fbec6d1571318c43259b2e4".to_string(),
                27
            ))
        );
    }
}