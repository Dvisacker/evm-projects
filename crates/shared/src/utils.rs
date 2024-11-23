use serde::Serialize;

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

#[cfg(test)]
mod tests {
    use serde::Serialize;

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
}
