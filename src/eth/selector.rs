use tiny_keccak::{Keccak, Hasher};
use hex;
pub fn handle_selector(func_sig: &str) {
    if !is_valid_func_sig(func_sig) {
        println!("Invalid function signature");
        return;
    }

    let selector = calc_selector(func_sig);
    println!("Selector: {}", selector);
}

pub fn calc_selector(func_sig: &str) -> String {
    let hash = keccak256(func_sig.as_bytes());
    let selector = &hash[0..4];
    format!("0x{}", hex::encode(selector))
}

fn keccak256(input: &[u8]) -> [u8; 32] {
    let mut hasher = Keccak::v256();
    let mut output = [0u8; 32];
    hasher.update(input);
    hasher.finalize(&mut output);
    output
}

fn is_valid_func_sig(func_sig: &str) -> bool {
    let re = regex::Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*\((([a-zA-Z0-9_\[\]]+)(,[a-zA-Z0-9_\[\]]+)*)?\)$").unwrap();
    re.is_match(func_sig)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_selector() {
        let selector = calc_selector("transfer(address,uint256)");
        assert_eq!(selector, "0xa9059cbb");
    }

    #[test]
    fn test_is_valid_func_sig() {
        assert_eq!(is_valid_func_sig("transfer(address,uint256)"), true);
        assert_eq!(is_valid_func_sig("function transfer(address,uint256)"), false);
        assert_eq!(is_valid_func_sig("transfer(address,uint256"), false);
        assert_eq!(is_valid_func_sig("transfer(address,uint256) "), false);
        assert_eq!(is_valid_func_sig("transfer(address,uint256)()"), false);
    }
}