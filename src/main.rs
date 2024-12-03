use std::collections::{BTreeMap, BTreeSet};
use std::io;

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Clone)]
enum Value {
    String(String),
    Integer(i32),
    Boolean(bool),
    Map(BTreeMap<String, Value>),
    List(Vec<Value>),
    Set(BTreeSet<Value>), // 追加
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
                    store.insert(key.to_string(), val.clone());
                    format!("Set key '{}' with value '{:?}'", key, val)
                },
                None => "Unsupported value type. Supported types: String, Integer, Boolean, Map, List, Set".to_string(),
            }
        }
        Some("get") => {
            let key = parts.next();
            match key {
                Some(k) => match store.get(k) {
                    Some(v) => format!("Value for key '{}': '{:?}'", k, v),
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
    } else if input.starts_with('[') && input.ends_with(']') {
        let inner = &input[1..input.len() - 1];
        let mut list = Vec::new();
        for item in split_list_items(inner) {
            let parsed_item = parse_value(item.trim())?;
            list.push(parsed_item);
        }
        Some(Value::List(list))
    } else if input.starts_with('<') && input.ends_with('>') {
        let inner = &input[1..input.len() - 1];
        let mut set = BTreeSet::new();
        for item in split_set_items(inner) {
            let parsed_item = parse_value(item.trim())?;
            set.insert(parsed_item);
        }
        Some(Value::Set(set))
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
            '{' | '[' | '<' => {
                brace_level += 1;
                current.push(c);
            }
            '}' | ']' | '>' => {
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

// リストの項目を正確に分割する関数
fn split_list_items(s: &str) -> Vec<String> {
    let mut items = Vec::new();
    let mut brace_level = 0;
    let mut current = String::new();

    for c in s.chars() {
        match c {
            '{' | '[' | '<' => {
                brace_level += 1;
                current.push(c);
            }
            '}' | ']' | '>' => {
                brace_level -= 1;
                current.push(c);
            }
            ',' if brace_level == 0 => {
                items.push(current.trim().to_string());
                current.clear();
            }
            _ => {
                current.push(c);
            }
        }
    }

    if !current.is_empty() {
        items.push(current.trim().to_string());
    }

    items
}

// Setの項目を正確に分割する関数
fn split_set_items(s: &str) -> Vec<String> {
    split_list_items(s) // 同じ分割ロジックを使用
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
        let _output = process_command(&mut store, "set key1 hello");
        assert_eq!(store.get("key1"), Some(&Value::String("hello".to_string())));

        let output = process_command(&mut store, "get key1");
        assert_eq!(output, "Value for key 'key1': 'String(\"hello\")'");
    }

    #[test]
    fn test_set_and_get_integer() {
        let mut store = BTreeMap::new();
        let _output = process_command(&mut store, "set key2 42");
        assert_eq!(store.get("key2"), Some(&Value::Integer(42)));

        let output = process_command(&mut store, "get key2");
        assert_eq!(output, "Value for key 'key2': 'Integer(42)'");
    }

    #[test]
    fn test_set_and_get_boolean() {
        let mut store = BTreeMap::new();
        let _output = process_command(&mut store, "set key3 true");
        assert_eq!(store.get("key3"), Some(&Value::Boolean(true)));

        let output = process_command(&mut store, "get key3");
        assert_eq!(output, "Value for key 'key3': 'Boolean(true)'");
    }

    #[test]
    fn test_set_and_get_map() {
        let mut store = BTreeMap::new();
        let _output = process_command(
            &mut store,
            r#"set key4 {"subkey1": "value1", "subkey2": 100}"#,
        );
        let mut expected_map = BTreeMap::new();
        expected_map.insert("subkey1".to_string(), Value::String("value1".to_string()));
        expected_map.insert("subkey2".to_string(), Value::Integer(100));
        assert_eq!(store.get("key4"), Some(&Value::Map(expected_map)));

        let output = process_command(&mut store, "get key4");
        assert_eq!(
            output,
            "Value for key 'key4': 'Map({\"subkey1\": String(\"value1\"), \"subkey2\": Integer(100)})'"
        );
    }

    #[test]
    fn test_set_and_get_list() {
        let mut store = BTreeMap::new();
        let _output = process_command(&mut store, r#"set key5 ["item1", 2, true]"#);
        let expected_list = vec![
            Value::String("item1".to_string()),
            Value::Integer(2),
            Value::Boolean(true),
        ];
        assert_eq!(store.get("key5"), Some(&Value::List(expected_list)));

        let output = process_command(&mut store, "get key5");
        assert_eq!(
            output,
            "Value for key 'key5': 'List([String(\"item1\"), Integer(2), Boolean(true)])'"
        );
    }

    #[test]
    fn test_set_and_get_set() {
        let mut store = BTreeMap::new();
        let _output = process_command(&mut store, r#"set key6 <"item1", 2, true>"#);
        let expected_set: BTreeSet<Value> = [
            Value::String("item1".to_string()),
            Value::Integer(2),
            Value::Boolean(true),
        ]
        .iter()
        .cloned()
        .collect();
        assert_eq!(store.get("key6"), Some(&Value::Set(expected_set)));

        let output = process_command(&mut store, "get key6");
        assert_eq!(
            output,
            "Value for key 'key6': 'Set({String(\"item1\"), Integer(2), Boolean(true)})'"
        );
    }

    #[test]
    fn test_set_unsupported_type() {
        let mut store = BTreeMap::new();
        let output = process_command(&mut store, "set key7 3.14");
        assert_eq!(
            output,
            "Unsupported value type. Supported types: String, Integer, Boolean, Map, List, Set"
        );
    }

    #[test]
    fn test_get_nonexistent_key() {
        let mut store = BTreeMap::new();
        let output = process_command(&mut store, "get key8");
        assert_eq!(output, "Key 'key8' not found");
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
        let output = process_command(&mut store, "set key9");
        assert_eq!(output, "Usage: set <key> <value>");
    }
}
