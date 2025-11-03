use assert_cmd::cargo::cargo_bin_cmd;

#[test]
fn test_selector() {
    let mut cmd = cargo_bin_cmd!("web3-tools");
    cmd.args(&["eth", "selector", "transfer(address,uint256)"]);
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("0xa9059cbb"));
}