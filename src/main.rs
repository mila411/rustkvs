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
        Some("exit") => "Exiting...".to_string(),
        Some(cmd) => format!(
            "Error: Unknown command '{}'.\nUse 'help' to see available commands.",
            cmd
        ),
        None => "Error: Please enter a command.\nUse 'help' to see available commands.".to_string(),
    }
}

fn parse_value(input: &str) -> Option<Value> {
    if let Ok(int_val) = input.parse::<i32>() {
        return Some(Value::Integer(int_val));
    }
    if input.eq_ignore_ascii_case("true") {
        return Some(Value::Boolean(true));
    }
    if input.eq_ignore_ascii_case("false") {
        return Some(Value::Boolean(false));
    }
    if input.starts_with('"') && input.ends_with('"') {
        return Some(Value::String(input.trim_matches('"').to_string()));
    }
    // Add parsing logic for other types
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_existing_key() {
        let mut store = BTreeMap::new();
        store.insert("key1".to_string(), Value::String("value1".to_string()));
        let history = Vec::new();
        let output = process_command(&mut store, "delete key1", &history);
        assert_eq!(output, "Key 'key1' deleted.");
        assert!(!store.contains_key("key1"));
    }

    #[test]
    fn test_delete_nonexistent_key() {
        let mut store = BTreeMap::new();
        let history = Vec::new();
        let output = process_command(&mut store, "delete key2", &history);
        assert_eq!(output, "Error: Key 'key2' was not found.");
    }

    #[test]
    fn test_delete_missing_argument() {
        let mut store = BTreeMap::new();
        let history = Vec::new();
        let output = process_command(&mut store, "delete", &history);
        assert_eq!(
            output,
            "Error: The 'delete' command requires <key>.\nUsage: delete <key>"
        );
    }

    #[test]
    fn test_update_existing_key() {
        let mut store = BTreeMap::new();
        store.insert("key1".to_string(), Value::String("old_value".to_string()));
        let history = Vec::new();
        let output = process_command(&mut store, "update key1 \"new_value\"", &history);
        assert_eq!(
            output,
            "Updated key 'key1' with new value 'String(\"new_value\")'"
        );
        assert_eq!(
            store.get("key1"),
            Some(&Value::String("new_value".to_string()))
        );
    }

    #[test]
    fn test_update_nonexistent_key() {
        let mut store = BTreeMap::new();
        let history = Vec::new();
        let output = process_command(&mut store, "update key2 100", &history);
        assert_eq!(output, "Error: Key 'key2' does not exist.");
    }

    #[test]
    fn test_update_missing_arguments() {
        let mut store = BTreeMap::new();
        let history = Vec::new();
        let output = process_command(&mut store, "update key3", &history);
        assert_eq!(
            output,
            "Error: The 'update' command requires <new_value>.\nUsage: update <key> <new_value>"
        );
    }

    #[test]
    fn test_list_empty_store() {
        let mut store = BTreeMap::new();
        let history = Vec::new();
        let output = process_command(&mut store, "list", &history);
        assert_eq!(output, "Store is empty.");
    }

    #[test]
    fn test_list_with_keys() {
        let mut store = BTreeMap::new();
        store.insert("key1".to_string(), Value::String("value1".to_string()));
        store.insert("key2".to_string(), Value::Integer(42));
        let history = Vec::new();
        let output = process_command(&mut store, "list", &history);
        assert_eq!(output, "Keys: key1, key2");
    }

    #[test]
    fn test_list_with_extra_arguments() {
        let mut store = BTreeMap::new();
        let history = Vec::new();
        let output = process_command(&mut store, "list extra_arg", &history);
        assert_eq!(
            output,
            "Error: The 'list' command does not take any arguments.\nUsage: list"
        );
    }

    #[test]
    fn test_help_command() {
        let mut store = BTreeMap::new();
        let history = Vec::new();
        let output = process_command(&mut store, "help", &history);
        let expected_output = "Available commands:\n\
                                - set <key> <value>: Sets a key-value pair in the store.\n\
                                - get <key>: Retrieves the value for the specified key.\n\
                                - delete <key>: Removes the specified key from the store.\n\
                                - update <key> <new_value>: Updates the value of an existing key.\n\
                                - list: Lists all keys currently in the store.\n\
                                - history: Displays the history of executed commands.\n\
                                - help: Displays available commands and their descriptions.\n\
                                - exit: Exits the program.";
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_help_with_extra_arguments() {
        let mut store = BTreeMap::new();
        let history = Vec::new();
        let output = process_command(&mut store, "help extra_arg", &history);
        assert_eq!(
            output,
            "Error: The 'help' command does not take any arguments.\nUsage: help"
        );
    }

    #[test]
    fn test_history_empty() {
        let mut store = BTreeMap::new();
        let history = Vec::new();
        let output = process_command(&mut store, "history", &history);
        assert_eq!(output, "No commands in history.");
    }

    #[test]
    fn test_history_with_commands() {
        let mut store = BTreeMap::new();
        let mut history = Vec::new();
        history.push("set key1 \"value1\"".to_string());
        history.push("get key1".to_string());
        let output = process_command(&mut store, "history", &history);
        let expected_output = "Command History:\n1: set key1 \"value1\"\n2: get key1";
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_history_with_extra_arguments() {
        let mut store = BTreeMap::new();
        let history = Vec::new();
        let output = process_command(&mut store, "history extra_arg", &history);
        assert_eq!(
            output,
            "Error: The 'history' command does not take any arguments.\nUsage: history"
        );
    }

    #[test]
    fn test_unknown_command() {
        let mut store = BTreeMap::new();
        let history = Vec::new();
        let output = process_command(&mut store, "unknown_cmd", &history);
        assert_eq!(
            output,
            "Error: Unknown command 'unknown_cmd'.\nUse 'help' to see available commands."
        );
    }

    #[test]
    fn test_empty_input() {
        let mut store = BTreeMap::new();
        let history = Vec::new();
        let output = process_command(&mut store, "", &history);
        assert_eq!(
            output,
            "Error: Please enter a command.\nUse 'help' to see available commands."
        );
    }

    #[test]
    fn test_set_and_get_integer() {
        let mut store = BTreeMap::new();
        let history = Vec::new();
        let output_set = process_command(&mut store, "set age 30", &history);
        assert_eq!(output_set, "Set key 'age' with value 'Integer(30)'");
        let output_get = process_command(&mut store, "get age", &history);
        assert_eq!(output_get, "Value for key 'age': 'Integer(30)'");
    }

    #[test]
    fn test_set_and_get_boolean() {
        let mut store = BTreeMap::new();
        let history = Vec::new();
        let output_set = process_command(&mut store, "set is_active true", &history);
        assert_eq!(output_set, "Set key 'is_active' with value 'Boolean(true)'");
        let output_get = process_command(&mut store, "get is_active", &history);
        assert_eq!(output_get, "Value for key 'is_active': 'Boolean(true)'");
    }

    #[test]
    fn test_set_and_get_string() {
        let mut store = BTreeMap::new();
        let history = Vec::new();
        let output_set = process_command(&mut store, "set greeting \"Hello\"", &history);
        assert_eq!(
            output_set,
            "Set key 'greeting' with value 'String(\"Hello\")'"
        );
        let output_get = process_command(&mut store, "get greeting", &history);
        assert_eq!(output_get, "Value for key 'greeting': 'String(\"Hello\")'");
    }
}
