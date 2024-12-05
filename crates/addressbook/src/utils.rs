pub fn get_workspace_dir() -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    let manifest_dir =
        std::env::var("CARGO_MANIFEST_DIR").map_err(|_| "CARGO_MANIFEST_DIR not set")?;
    let mut current_dir = std::path::PathBuf::from(manifest_dir);

    loop {
        let cargo_toml = current_dir.join("Cargo.toml");
        if cargo_toml.exists() {
            // Read the Cargo.toml file
            let contents = std::fs::read_to_string(&cargo_toml)?;
            if contents.contains("[workspace]") {
                return Ok(current_dir);
            }
        }

        // Move up one directory
        if !current_dir.pop() {
            break;
        }
    }

    Err("Could not find workspace root".into())
}
