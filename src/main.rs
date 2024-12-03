use std::collections::HashMap;
use std::io;

fn main() {
    let mut store = HashMap::new();

    loop {
        println!("Enter command:");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let output = process_command(&mut store, &input);
        println!("{}", output);

        if input.trim() == "exit" {
            break;
        }
    }
}

fn process_command(store: &mut HashMap<String, String>, input: &str) -> String {
    let mut parts = input.trim().split_whitespace();
    let command = parts.next();

    match command {
        Some("set") => {
            let key = parts.next();
            let value = parts.next();
            match (key, value) {
                (Some(k), Some(v)) => {
                    store.insert(k.to_string(), v.to_string());
                    format!("Set key '{}' with value '{}'", k, v)
                }
                _ => "Usage: set <key> <value>".to_string(),
            }
        }
        Some("get") => {
            let key = parts.next();
            match key {
                Some(k) => match store.get(k) {
                    Some(v) => format!("Value for key '{}': '{}'", k, v),
                    None => format!("Key '{}' not found", k),
                },
                None => "Usage: get <key>".to_string(),
            }
        }
        Some("exit") => "Exiting...".to_string(),
        _ => "Unknown command".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_and_get() {
        let mut store = HashMap::new();
        let output = process_command(&mut store, "set key1 value1");
        assert_eq!(output, "Set key 'key1' with value 'value1'");

        let output = process_command(&mut store, "get key1");
        assert_eq!(output, "Value for key 'key1': 'value1'");
    }

    #[test]
    fn test_get_nonexistent_key() {
        let mut store = HashMap::new();
        let output = process_command(&mut store, "get key2");
        assert_eq!(output, "Key 'key2' not found");
    }

    #[test]
    fn test_unknown_command() {
        let mut store = HashMap::new();
        let output = process_command(&mut store, "unknown");
        assert_eq!(output, "Unknown command");
    }

    #[test]
    fn test_set_missing_arguments() {
        let mut store = HashMap::new();
        let output = process_command(&mut store, "set key1");
        assert_eq!(output, "Usage: set <key> <value>");
    }
}
