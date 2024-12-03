use std::collections::{BTreeMap, BTreeSet};
use std::io;

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Clone)]
enum Value {
    String(String),
    Integer(i32),
    Boolean(bool),
    Map(BTreeMap<String, Value>),
    List(Vec<Value>),
    Set(BTreeSet<Value>),
}

fn main() {
    let mut store: BTreeMap<String, Value> = BTreeMap::new();
    let mut history: Vec<String> = Vec::new();

    loop {
        println!("Enter command:");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let trimmed_input = input.trim();
        if !trimmed_input.is_empty() {
            history.push(trimmed_input.to_string());
        }

        let output = process_command(&mut store, trimmed_input, &history);
        println!("{}", output);
        println!();

        if trimmed_input == "exit" {
            break;
        }
    }
}

fn process_command(
    store: &mut BTreeMap<String, Value>,
    input: &str,
    history: &Vec<String>,
) -> String {
    let mut parts = input.split_whitespace();
    let command = parts.next();

    match command {
        Some("set") => {
            let key = parts.next();
            let value: Vec<&str> = parts.collect();
            if key.is_none() && value.is_empty() {
                return "Error: The 'set' command requires <key> and <value>.\nUsage: set <key> <value>".to_string();
            } else if key.is_none() {
                return "Error: The 'set' command requires <key>.\nUsage: set <key> <value>"
                    .to_string();
            } else if value.is_empty() {
                return "Error: The 'set' command requires <value>.\nUsage: set <key> <value>"
                    .to_string();
            }
            let key = key.unwrap();
            let value_str = value.join(" ");
            let parsed_value = parse_value(&value_str);
            match parsed_value {
                Some(val) => {
                    store.insert(key.to_string(), val.clone());
                    format!("Set key '{}' with value '{:?}'", key, val)
                }
                None => "Error: Unsupported value type.\nSupported types: String, Integer, Boolean, Map, List, Set".to_string(),
            }
        }
        Some("get") => {
            let key = parts.next();
            match key {
                Some(k) => match store.get(k) {
                    Some(v) => format!("Value for key '{}': '{:?}'", k, v),
                    None => format!("Error: Key '{}' was not found.", k),
                },
                None => "Error: The 'get' command requires <key>.\nUsage: get <key>".to_string(),
            }
        }
        Some("delete") => {
            let key = parts.next();
            match key {
                Some(k) => {
                    if store.remove(k).is_some() {
                        format!("Key '{}' deleted.", k)
                    } else {
                        format!("Error: Key '{}' was not found.", k)
                    }
                }
                None => {
                    "Error: The 'delete' command requires <key>.\nUsage: delete <key>".to_string()
                }
            }
        }
        Some("update") => {
            let key = parts.next();
            let value: Vec<&str> = parts.collect();
            if key.is_none() && value.is_empty() {
                return "Error: The 'update' command requires <key> and <new_value>.\nUsage: update <key> <new_value>".to_string();
            } else if key.is_none() {
                return "Error: The 'update' command requires <key>.\nUsage: update <key> <new_value>".to_string();
            } else if value.is_empty() {
                return "Error: The 'update' command requires <new_value>.\nUsage: update <key> <new_value>".to_string();
            }
            let key = key.unwrap();
            if !store.contains_key(key) {
                return format!("Error: Key '{}' does not exist.", key);
            }
            let value_str = value.join(" ");
            let parsed_value = parse_value(&value_str);
            match parsed_value {
                Some(val) => {
                    store.insert(key.to_string(), val.clone());
                    format!("Updated key '{}' with new value '{:?}'", key, val)
                }
                None => "Error: Unsupported value type.\nSupported types: String, Integer, Boolean, Map, List, Set".to_string(),
            }
        }
        Some("list") => {
            let extra_args: Vec<&str> = parts.collect();
            if !extra_args.is_empty() {
                return "Error: The 'list' command does not take any arguments.\nUsage: list"
                    .to_string();
            }
            if store.is_empty() {
                "Store is empty.".to_string()
            } else {
                let keys: Vec<&str> = store.keys().map(|s| s.as_str()).collect();
                format!("Keys: {}", keys.join(", "))
            }
        }
        Some("history") => {
            let extra_args: Vec<&str> = parts.collect();
            if !extra_args.is_empty() {
                return "Error: The 'history' command does not take any arguments.\nUsage: history"
                    .to_string();
            }
            if history.is_empty() {
                "No commands in history.".to_string()
            } else {
                let history_output = history
                    .iter()
                    .enumerate()
                    .map(|(i, cmd)| format!("{}: {}", i + 1, cmd))
                    .collect::<Vec<String>>()
                    .join("\n");
                format!("Command History:\n{}", history_output)
            }
        }
        Some("help") => {
            let extra_args: Vec<&str> = parts.collect();
            if !extra_args.is_empty() {
                return "Error: The 'help' command does not take any arguments.\nUsage: help"
                    .to_string();
            }
            format!(
                "Available commands:\n\
                - set <key> <value>: Sets a key-value pair in the store.\n\
                - get <key>: Retrieves the value for the specified key.\n\
                - delete <key>: Removes the specified key from the store.\n\
                - update <key> <new_value>: Updates the value of an existing key.\n\
                - list: Lists all keys currently in the store.\n\
                - history: Displays the history of executed commands.\n\
                - help: Displays available commands and their descriptions.\n\
                - exit: Exits the program."
            )
        }
        Some("exit") => "Exiting...".to_string(),
        Some(cmd) => format!(
            "Error: Unknown command '{}'.\nUse 'help' to see available commands.",
            cmd
        ),
        None => "Error: Please enter a command.\nUse 'help' to see available commands.".to_string(),
    }
}

fn parse_value(input: &str) -> Option<Value> {
    let input = input.trim();

    // Integer
    if let Ok(int_val) = input.parse::<i32>() {
        return Some(Value::Integer(int_val));
    }

    // Boolean
    if input.eq_ignore_ascii_case("true") {
        return Some(Value::Boolean(true));
    }
    if input.eq_ignore_ascii_case("false") {
        return Some(Value::Boolean(false));
    }

    // String (When surrounded by double quotes)
    if input.starts_with('"') && input.ends_with('"') {
        return Some(Value::String(input.trim_matches('"').to_string()));
    }

    // Map
    if input.starts_with('{') && input.ends_with('}') {
        let inner = &input[1..input.len() - 1];
        let mut map = BTreeMap::new();
        for pair in split_top_level_commas(inner) {
            let parts: Vec<&str> = pair.splitn(2, ':').collect();
            if parts.len() != 2 {
                return None;
            }
            let key = parts[0].trim().trim_matches('"').to_string();
            let value = parse_value(parts[1].trim())?;
            map.insert(key, value);
        }
        return Some(Value::Map(map));
    }

    // List
    if input.starts_with('[') && input.ends_with(']') {
        let inner = &input[1..input.len() - 1];
        let list = inner
            .split(',')
            .map(|s| parse_value(s.trim()))
            .collect::<Option<Vec<Value>>>()?;
        return Some(Value::List(list));
    }

    // Set
    if input.starts_with('<') && input.ends_with('>') {
        let inner = &input[1..input.len() - 1];
        let set = inner
            .split(',')
            .map(|s| parse_value(s.trim()))
            .collect::<Option<BTreeSet<Value>>>()?;
        return Some(Value::Set(set));
    }

    None
}

// Top-level function to split by commas
fn split_top_level_commas(s: &str) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut bracket_stack = Vec::new();
    let mut last = 0;

    for (i, c) in s.char_indices() {
        match c {
            '{' | '[' | '<' => bracket_stack.push(c),
            '}' => {
                if bracket_stack.pop() != Some('{') {
                    return vec![];
                }
            }
            ']' => {
                if bracket_stack.pop() != Some('[') {
                    return vec![];
                }
            }
            '>' => {
                if bracket_stack.pop() != Some('<') {
                    return vec![];
                }
            }
            ',' => {
                if bracket_stack.is_empty() {
                    parts.push(&s[last..i]);
                    last = i + 1;
                }
            }
            _ => {}
        }
    }

    parts.push(&s[last..]);
    parts
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    #[test]
    fn test_set_and_get_map() {
        let mut store = BTreeMap::new();
        let history = Vec::new();
        let output_set = process_command(
            &mut store,
            r#"set user {"name":"Alice","age":30,"is_active":true}"#,
            &history,
        );
        assert_eq!(
            output_set,
            "Set key 'user' with value 'Map({\"age\": Integer(30), \"is_active\": Boolean(true), \"name\": String(\"Alice\")})'"
        );
        let output_get = process_command(&mut store, "get user", &history);
        assert_eq!(
            output_get,
            "Value for key 'user': 'Map({\"age\": Integer(30), \"is_active\": Boolean(true), \"name\": String(\"Alice\")})'"
        );
    }

    #[test]
    fn test_set_and_get_list() {
        let mut store = BTreeMap::new();
        let history = Vec::new();
        let output_set = process_command(&mut store, "set numbers [1, 2, 3, 4, 5]", &history);
        assert_eq!(
            output_set,
            "Set key 'numbers' with value 'List([Integer(1), Integer(2), Integer(3), Integer(4), Integer(5)])'"
        );
        let output_get = process_command(&mut store, "get numbers", &history);
        assert_eq!(
            output_get,
            "Value for key 'numbers': 'List([Integer(1), Integer(2), Integer(3), Integer(4), Integer(5)])'"
        );
    }

    #[test]
    fn test_set_and_get_set() {
        let mut store = BTreeMap::new();
        let history = Vec::new();
        let output_set = process_command(
            &mut store,
            r#"set unique_items <"apple", "banana", "cherry">"#,
            &history,
        );
        assert_eq!(
            output_set,
            "Set key 'unique_items' with value 'Set({String(\"apple\"), String(\"banana\"), String(\"cherry\")})'"
        );
        let output_get = process_command(&mut store, "get unique_items", &history);
        assert_eq!(
            output_get,
            "Value for key 'unique_items': 'Set({String(\"apple\"), String(\"banana\"), String(\"cherry\")})'"
        );
    }

    #[test]
    fn test_invalid_map_format() {
        let mut store = BTreeMap::new();
        let history = Vec::new();
        let output_set = process_command(
            &mut store,
            r#"set user {"name":"Alice","age":30,"is_active":true"#, // Missing closing brace
            &history,
        );
        assert_eq!(
            output_set,
            "Error: Unsupported value type.\nSupported types: String, Integer, Boolean, Map, List, Set".to_string()
        );
    }

    #[test]
    fn test_invalid_set_format() {
        let mut store = BTreeMap::new();
        let history = Vec::new();
        let output_set = process_command(
            &mut store,
            r#"set unique_items <"apple", "banana", "cherry""#, // Missing closing angle bracket
            &history,
        );
        assert_eq!(
            output_set,
            "Error: Unsupported value type.\nSupported types: String, Integer, Boolean, Map, List, Set".to_string()
        );
    }
}
