use tiny_keccak::{Keccak, Hasher};
use hex;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_selector() {
        let selector = calc_selector("transfer(address,uint256)");
        assert_eq!(selector, "0xa9059cbb");
    }
}