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
                return "Error: 'set' コマンドには <key> と <value> が必要です。\n使用方法: set <key> <value>".to_string();
            } else if key.is_none() {
                return "Error: 'set' コマンドには <key> が必要です。\n使用方法: set <key> <value>"
                    .to_string();
            } else if value.is_empty() {
                return "Error: 'set' コマンドには <value> が必要です。\n使用方法: set <key> <value>".to_string();
            }
            let key = key.unwrap();
            let value_str = value.join(" ");
            let parsed_value = parse_value(&value_str);
            match parsed_value {
                Some(val) => {
                    store.insert(key.to_string(), val.clone());
                    format!("Set key '{}' with value '{:?}'", key, val)
                }
                None => "Error: サポートされていない値の型です。\nサポートされている型: String, Integer, Boolean, Map, List, Set".to_string(),
            }
        }
        Some("get") => {
            let key = parts.next();
            match key {
                Some(k) => match store.get(k) {
                    Some(v) => format!("Value for key '{}': '{:?}'", k, v),
                    None => format!("Error: Key '{}' が見つかりません。", k),
                },
                None => {
                    "Error: 'get' コマンドには <key> が必要です。\n使用方法: get <key>".to_string()
                }
            }
        }
        Some("delete") => {
            let key = parts.next();
            match key {
                Some(k) => {
                    if store.remove(k).is_some() {
                        format!("Key '{}' deleted.", k)
                    } else {
                        format!("Error: Key '{}' が見つかりません。", k)
                    }
                }
                None => "Error: 'delete' コマンドには <key> が必要です。\n使用方法: delete <key>"
                    .to_string(),
            }
        }
        Some("update") => {
            let key = parts.next();
            let value: Vec<&str> = parts.collect();
            if key.is_none() && value.is_empty() {
                return "Error: 'update' コマンドには <key> と <new_value> が必要です。\n使用方法: update <key> <new_value>".to_string();
            } else if key.is_none() {
                return "Error: 'update' コマンドには <key> が必要です。\n使用方法: update <key> <new_value>".to_string();
            } else if value.is_empty() {
                return "Error: 'update' コマンドには <new_value> が必要です。\n使用方法: update <key> <new_value>".to_string();
            }
            let key = key.unwrap();
            if !store.contains_key(key) {
                return format!("Error: Key '{}' は存在しません。", key);
            }
            let value_str = value.join(" ");
            let parsed_value = parse_value(&value_str);
            match parsed_value {
                Some(val) => {
                    store.insert(key.to_string(), val.clone());
                    format!("Updated key '{}' with new value '{:?}'", key, val)
                }
                None => "Error: サポートされていない値の型です。\nサポートされている型: String, Integer, Boolean, Map, List, Set".to_string(),
            }
        }
        Some("list") => {
            let extra_args: Vec<&str> = parts.collect();
            if !extra_args.is_empty() {
                return "Error: 'list' コマンドには引数が不要です。\n使用方法: list".to_string();
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
                return "Error: 'help' コマンドには引数が不要です。\n使用方法: help".to_string();
            }
            format!(
                "利用可能なコマンド:\n\
                - set <key> <value>: キーと値をストアに設定します。\n\
                - get <key>: 指定したキーの値を取得します。\n\
                - delete <key>: 指定したキーをストアから削除します。\n\
                - update <key> <new_value>: 既存のキーの値を更新します。\n\
                - list: 現在保存されている全てのキーを一覧表示します。\n\
                - history: 実行したコマンドの履歴を表示します。\n\
                - help: 利用可能なコマンドとその説明を表示します。\n\
                - exit: プログラムを終了します。"
            )
        }
        Some("history") => {
            let extra_args: Vec<&str> = parts.collect();
            if !extra_args.is_empty() {
                return "Error: 'history' コマンドには引数が不要です。\n使用方法: history"
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
        None => {
            "Error: コマンドを入力してください。\nUse 'help' to see available commands.".to_string()
        }
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
    // 他の型のパースロジックを追加
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
        assert_eq!(output, "Error: Key 'key2' が見つかりません。");
    }

    #[test]
    fn test_delete_missing_argument() {
        let mut store = BTreeMap::new();
        let history = Vec::new();
        let output = process_command(&mut store, "delete", &history);
        assert_eq!(
            output,
            "Error: 'delete' コマンドには <key> が必要です。\n使用方法: delete <key>"
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
        assert_eq!(output, "Error: Key 'key2' は存在しません。");
    }

    #[test]
    fn test_update_missing_arguments() {
        let mut store = BTreeMap::new();
        let history = Vec::new();
        let output = process_command(&mut store, "update key3", &history);
        assert_eq!(
            output,
            "Error: 'update' コマンドには <new_value> が必要です。\n使用方法: update <key> <new_value>"
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
            "Error: 'list' コマンドには引数が不要です。\n使用方法: list"
        );
    }

    #[test]
    fn test_help_command() {
        let mut store = BTreeMap::new();
        let history = Vec::new();
        let output = process_command(&mut store, "help", &history);
        let expected_output = "利用可能なコマンド:\n\
                                - set <key> <value>: キーと値をストアに設定します。\n\
                                - get <key>: 指定したキーの値を取得します。\n\
                                - delete <key>: 指定したキーをストアから削除します。\n\
                                - update <key> <new_value>: 既存のキーの値を更新します。\n\
                                - list: 現在保存されている全てのキーを一覧表示します。\n\
                                - history: 実行したコマンドの履歴を表示します。\n\
                                - help: 利用可能なコマンドとその説明を表示します。\n\
                                - exit: プログラムを終了します。";
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_help_with_extra_arguments() {
        let mut store = BTreeMap::new();
        let history = Vec::new();
        let output = process_command(&mut store, "help extra_arg", &history);
        assert_eq!(
            output,
            "Error: 'help' コマンドには引数が不要です。\n使用方法: help"
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
            "Error: 'history' コマンドには引数が不要です。\n使用方法: history"
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
            "Error: コマンドを入力してください。\nUse 'help' to see available commands."
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
        let output_set = process_command(&mut store, "set greeting \"こんにちは\"", &history);
        assert_eq!(
            output_set,
            "Set key 'greeting' with value 'String(\"こんにちは\")'"
        );
        let output_get = process_command(&mut store, "get greeting", &history);
        assert_eq!(
            output_get,
            "Value for key 'greeting': 'String(\"こんにちは\")'"
        );
    }
}
