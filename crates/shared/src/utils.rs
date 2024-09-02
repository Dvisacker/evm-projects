pub fn bytes32_to_string(bytes: &[u8]) -> String {
    let mut result = String::from_utf8_lossy(bytes).into_owned();
    result.truncate(result.trim_end_matches('\0').len());
    result
}
