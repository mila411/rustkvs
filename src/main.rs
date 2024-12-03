use std::collections::HashMap;
use std::io;

#[derive(Debug, PartialEq)]
enum Value {
    String(String),
    Integer(i32),
    Boolean(bool),
}

fn main() {
    let mut store: HashMap<String, Value> = HashMap::new();

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

fn process_command(store: &mut HashMap<String, Value>, input: &str) -> String {
    let mut parts = input.trim().split_whitespace();
    let command = parts.next();

    match command {
        Some("set") => {
            let key = parts.next();
            let value = parts.next();
            match (key, value) {
                (Some(k), Some(v)) => {
                    let parsed_value = parse_value(v);
                    match parsed_value {
                        Some(val) => {
                            store.insert(k.to_string(), val);
                            format!("Set key '{}' with value '{:?}'", k, store.get(k))
                        }
                        None => "Unsupported value type. Supported types: String, Integer, Boolean"
                            .to_string(),
                    }
                }
                _ => "Usage: set <key> <value>".to_string(),
            }
        }
        Some("get") => {
            let key = parts.next();
            match key {
                Some(k) => match store.get(k) {
                    Some(v) => format!("Value for key '{}': '{:?}'", k, store.get(k)),
                    None => format!("Key '{}' not found", k),
                },
                None => "Usage: get <key>".to_string(),
            }
        }
        Some("exit") => "Exiting...".to_string(),
        _ => "Unknown command".to_string(),
    }
}

fn parse_value(input: &str) -> Option<Value> {
    if let Ok(i) = input.parse::<i32>() {
        Some(Value::Integer(i))
    } else if let Ok(b) = input.parse::<bool>() {
        Some(Value::Boolean(b))
    } else if input.parse::<f64>().is_err() {
        // Float 型をサポート外とする
        Some(Value::String(input.to_string()))
    } else {
        None // サポート外の型
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_and_get_string() {
        let mut store = HashMap::new();
        let output = process_command(&mut store, "set key1 hello");
        assert_eq!(
            output,
            "Set key 'key1' with value 'Some(String(\"hello\"))'"
        );

        let output = process_command(&mut store, "get key1");
        assert_eq!(output, "Value for key 'key1': 'Some(String(\"hello\"))'");
    }

    #[test]
    fn test_set_and_get_integer() {
        let mut store = HashMap::new();
        let output = process_command(&mut store, "set key2 42");
        assert_eq!(output, "Set key 'key2' with value 'Some(Integer(42))'");

        let output = process_command(&mut store, "get key2");
        assert_eq!(output, "Value for key 'key2': 'Some(Integer(42))'");
    }

    #[test]
    fn test_set_and_get_boolean() {
        let mut store = HashMap::new();
        let output = process_command(&mut store, "set key3 true");
        assert_eq!(output, "Set key 'key3' with value 'Some(Boolean(true))'");

        let output = process_command(&mut store, "get key3");
        assert_eq!(output, "Value for key 'key3': 'Some(Boolean(true))'");
    }

    #[test]
    fn test_set_unsupported_type() {
        let mut store = HashMap::new();
        let output = process_command(&mut store, "set key5 3.14");
        assert_eq!(
            output,
            "Unsupported value type. Supported types: String, Integer, Boolean"
        );
    }

    #[test]
    fn test_get_nonexistent_key() {
        let mut store = HashMap::new();
        let output = process_command(&mut store, "get key4");
        assert_eq!(output, "Key 'key4' not found");
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
