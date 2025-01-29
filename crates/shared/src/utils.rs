use alloy_primitives::Address;
use eyre::Result;
use serde::Serialize;
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;

pub fn bytes32_to_string(bytes: &[u8]) -> String {
    let mut result = String::from_utf8_lossy(bytes).into_owned();
    result.truncate(result.trim_end_matches('\0').len());
    result
}

pub fn prettify<T: std::fmt::Debug>(value: &T) -> String {
    format!("{:#?}", value)
}

pub fn prettify_json<T: Serialize>(value: &T) -> String {
    serde_json::to_string_pretty(value)
        .unwrap_or_else(|_| String::from("Error serializing to JSON"))
}

fn read_dir_recursive(path: &Path, depth: u32, files: &mut Vec<PathBuf>) -> std::io::Result<()> {
    if depth == 0 {
        return Ok(());
    }

    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                read_dir_recursive(&path, depth - 1, files)?;
            } else {
                files.push(path);
            }
        }
    }
    Ok(())
}

pub fn get_most_recent_deployment(
    contract_name: &str,
    chain_id: u64,
    path: Option<PathBuf>,
) -> Result<Address> {
    let mut latest_address = Address::ZERO;
    let mut last_timestamp = 0u64;
    let mut run_processed = false;

    // Read all files up to 3 levels deep in the foundry broadcast directory
    let mut files = Vec::new();
    read_dir_recursive(&path.unwrap(), 3, &mut files)?;

    for path in files {
        if !is_valid_deployment_file(&path, chain_id) {
            continue;
        }

        run_processed = true;
        let json = fs::read_to_string(&path)?;
        let json: Value = serde_json::from_str(&json)?;
        let timestamp = json["timestamp"].as_u64().unwrap_or(0);

        if timestamp > last_timestamp {
            if let Some(address) = process_run(&json, contract_name) {
                latest_address = address;
                last_timestamp = timestamp;
            }
        }
    }

    if !run_processed {
        eyre::bail!("No deployment artifacts were found for specified chain");
    }

    if latest_address != Address::ZERO {
        Ok(latest_address)
    } else {
        eyre::bail!(
            "No contract named '{}' has been deployed on chain {}",
            contract_name,
            chain_id
        )
    }
}

fn is_valid_deployment_file(path: &Path, chain_id: u64) -> bool {
    let path_str = path.to_string_lossy();
    path_str.contains(&format!("/{}/", chain_id))
        && path_str.ends_with(".json")
        && !path_str.contains("dry-run")
}

fn process_run(json: &Value, contract_name: &str) -> Option<Address> {
    if let Some(transactions) = json["transactions"].as_array() {
        for tx in transactions {
            if let Some(deployed_contract_name) = tx["contractName"].as_str() {
                if deployed_contract_name == contract_name {
                    if let Some(address) = tx["contractAddress"].as_str() {
                        return Some(Address::from_str(address).ok()?);
                    }
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use serde::Serialize;
    use std::env;

    #[derive(Serialize, Debug)]
    struct Person {
        name: String,
        age: u8,
    }

    use super::*;

    #[tokio::test]
    async fn test_pretty_print() {
        // Example with a JSON-serializable struct
        let person = Person {
            name: String::from("John"),
            age: 30,
        };

        println!("{}", prettify(&person));
        println!("{}", prettify_json(&person));
    }

    #[test]
    fn test_get_most_recent_deployment() {
        println!("current dir: {:?}", env::current_dir().unwrap());
        env::set_var("BROADCAST_PATH", "../../contracts/broadcast");
        // You'll need to adjust this test based on your actual broadcast files
        let result = get_most_recent_deployment("BatchExecutor", 8453, None);
        println!("result: {:?}", result);
        assert!(result.is_ok());

        let address = result.unwrap();
        println!("Most recent deployment address: {:#x}", address);
    }

    #[test]
    fn test_invalid_chain() {
        let result = get_most_recent_deployment("BatchExecutor", 999999, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_contract() {
        let result = get_most_recent_deployment("NonExistentContract", 8453, None);
        assert!(result.is_err());
    }
}
