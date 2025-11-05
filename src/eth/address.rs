use ethers::{prelude::*};
pub fn handle_address(private_key: &str) -> String {
    if !is_valid_private_key(private_key) {
        println!("Invalid private key");
        return "".to_string();
    }

    let pk = if private_key.starts_with("0x") {
        &private_key[2..]
    } else {
        private_key
    };

    let wallet: LocalWallet = match pk.parse() {
        Ok(w) => w,
        Err(e) => {
            println!("Error parsing private key: {}", e);
            return "".to_string();
        }
    };
    
    let address = hex::encode(wallet.address().as_bytes());

    println!("Address: 0x{}", address);

    format!("0x{}", address)
}

pub fn is_valid_private_key(s: &str) -> bool {
    let pk = if s.starts_with("0x") {
        &s[2..]
    } else {
        s
    };

    // 私钥长度为64位
    if pk.len() != 64 {
        return false;
    }

    // 私钥是否为16进制数
    for c in pk.chars() {
        if !c.is_ascii_hexdigit() {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_private_key() {
        assert_eq!(is_valid_private_key("0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"), true);
        assert_eq!(is_valid_private_key("0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdex"), false);
        assert_eq!(is_valid_private_key("0x123"), false);
    }

    #[test]
    fn test_pk_to_address() {
        assert_eq!(
            handle_address("0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"),
            "0x1be31a94361a391bbafb2a4ccd704f57dc04d4bb"
        );

        assert_eq!(
            handle_address("0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdex"),
            ""
        );
    }
}