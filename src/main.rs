use std::collections::BTreeMap;
use std::io;

#[derive(Debug, PartialEq)]
enum Value {
    String(String),
    Integer(i32),
    Boolean(bool),
    Map(BTreeMap<String, Value>),
}

fn main() {
    let mut store: BTreeMap<String, Value> = BTreeMap::new();

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

fn process_command(store: &mut BTreeMap<String, Value>, input: &str) -> String {
    let mut parts = input.trim().split_whitespace();
    let command = parts.next();

    match command {
        Some("set") => {
            let key = parts.next();
            let value: Vec<&str> = parts.collect();
            if key.is_none() || value.is_empty() {
                return "Usage: set <key> <value>".to_string();
            }
            let key = key.unwrap();
            let value_str = value.join(" ");
            let parsed_value = parse_value(&value_str);
            match parsed_value {
                Some(val) => {
                    store.insert(key.to_string(), val);
                    format!("Set key '{}' with value '{:?}'", key, store.get(key))
                }
                None => "Unsupported value type. Supported types: String, Integer, Boolean, Map"
                    .to_string(),
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
    let input = input.trim();
    if let Ok(i) = input.parse::<i32>() {
        Some(Value::Integer(i))
    } else if let Ok(b) = input.parse::<bool>() {
        Some(Value::Boolean(b))
    } else if input.starts_with('{') && input.ends_with('}') {
        let inner = &input[1..input.len() - 1];
        let mut map = BTreeMap::new();
        for pair in split_pairs(inner) {
            let mut kv = pair.splitn(2, ':');
            if let (Some(k), Some(v)) = (kv.next(), kv.next()) {
                let key = trim_quotes(k.trim());
                let value = parse_value(v.trim())?;
                map.insert(key, value);
            } else {
                return None;
            }
        }
        Some(Value::Map(map))
    } else if input.parse::<f64>().is_err() {
        Some(Value::String(trim_quotes(input)))
    } else {
        None
    }
}

// キー・バリューのペアを正確に分割する関数
fn split_pairs(s: &str) -> Vec<String> {
    let mut pairs = Vec::new();
    let mut brace_level = 0;
    let mut current = String::new();

    for c in s.chars() {
        match c {
            '{' => {
                brace_level += 1;
                current.push(c);
            }
            '}' => {
                brace_level -= 1;
                current.push(c);
            }
            ',' if brace_level == 0 => {
                pairs.push(current.trim().to_string());
                current.clear();
            }
            _ => {
                current.push(c);
            }
        }
    }

    if !current.is_empty() {
        pairs.push(current.trim().to_string());
    }

    pairs
}

// 引用符を削除する関数
fn trim_quotes(s: &str) -> String {
    s.trim_matches('"').to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_and_get_string() {
        let mut store = BTreeMap::new();
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
        let mut store = BTreeMap::new();
        let output = process_command(&mut store, "set key2 42");
        assert_eq!(output, "Set key 'key2' with value 'Some(Integer(42))'");

        let output = process_command(&mut store, "get key2");
        assert_eq!(output, "Value for key 'key2': 'Some(Integer(42))'");
    }

    #[test]
    fn test_set_and_get_boolean() {
        let mut store = BTreeMap::new();
        let output = process_command(&mut store, "set key3 true");
        assert_eq!(output, "Set key 'key3' with value 'Some(Boolean(true))'");

        let output = process_command(&mut store, "get key3");
        assert_eq!(output, "Value for key 'key3': 'Some(Boolean(true))'");
    }

    #[test]
    fn test_set_unsupported_type() {
        let mut store = BTreeMap::new();
        let output = process_command(&mut store, "set key5 3.14");
        assert_eq!(
            output,
            "Unsupported value type. Supported types: String, Integer, Boolean, Map"
        );
    }

    #[test]
    fn test_get_nonexistent_key() {
        let mut store = BTreeMap::new();
        let output = process_command(&mut store, "get key4");
        assert_eq!(output, "Key 'key4' not found");
    }

    #[test]
    fn test_unknown_command() {
        let mut store = BTreeMap::new();
        let output = process_command(&mut store, "unknown");
        assert_eq!(output, "Unknown command");
    }

    #[test]
    fn test_set_missing_arguments() {
        let mut store = BTreeMap::new();
        let output = process_command(&mut store, "set key1");
        assert_eq!(output, "Usage: set <key> <value>");
    }

    #[test]
    fn test_set_and_get_map() {
        let mut store = BTreeMap::new();
        let output = process_command(
            &mut store,
            r#"set key4 {"subkey1": "value1", "subkey2": 100}"#,
        );
        assert_eq!(
            output,
            "Set key 'key4' with value 'Some(Map({\"subkey1\": String(\"value1\"), \"subkey2\": Integer(100)}))'"
        );

        let output = process_command(&mut store, "get key4");
        assert_eq!(
            output,
            "Value for key 'key4': 'Some(Map({\"subkey1\": String(\"value1\"), \"subkey2\": Integer(100)}))'"
        );
    }
}
