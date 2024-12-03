# Rust Key-Value Store

⚠️⚠️⚠️<br>
This repository is for learning and enjoying low-level programming in Rust without using external crates. Please note that it lacks the functionality required for use in a production ready.<br>
⚠️⚠️⚠️

## Description

This CLI-based Key-Value Store, built with Rust, supports various data types including `String`, `Integer`, `Boolean`, `Map`, `List`, and `Set`. Users can perform operations such as setting, retrieving, updating, and deleting data directly from the command line. The data is not persisted and is handled entirely in memory.

## Installation

### Prerequisites

- **Rust**: Ensure that Rust is installed on your system. If not, install it from [rust-lang.org](https://www.rust-lang.org/tools/install).

### Steps

1. **Clone the Repository**

`git clone https://github.com/mila411/rustkvs`

2. **Navigate to the Project Directory**

`cd key-value-store`

3. Build the Project

`cargo build --release`

4. Run the Application

`cargo run`

### Usage

After running the application, you'll be prompted to enter commands. Below are the available commands and their descriptions.

#### Commands

`set`

- **Description:** Sets a key-value pair in the store.
- **Usage:** `set <key> <value>`
- **Supported Data Types:** `String`, `Integer`, `Boolean`, `Map`, `List`, `Set`

`get`

- **Description:** Retrieves the value for the specified key.
- **Usage:** `get <key>`

`delete`

- **Description:** Removes the specified key from the store.
- **Usage:** `delete <key>`

`update`

- **Description:** Updates the value of an existing key.
- **Usage:** `update <key> <new_value>`

`list`

- **Description:** Lists all keys currently in the store.
- **Usage:** `list`

`history`

- **Description:** Displays the history of executed commands.
- **Usage:** `history`

`help`

- **Description:** Displays available commands and their descriptions.
- **Usage:** `help`

`exit`

- **Description:** Exits the program.
- **Usage:** `exit`

### Examples

Setting a String

```sh
Enter command:
set username "Alice"
Set key 'username' with value 'String("Alice")'
```

Setting an Integer

```sh
Enter command:
set age 30
Set key 'age' with value 'Integer(30)'
```

Setting a Boolean

```sh
Enter command:
set is_active true
Set key 'is_active' with value 'Boolean(true)'
```

Setting a Map

```sh
Enter command:
set user {"name":"Alice","age":30,"is_active":true}
Set key 'user' with value 'Map({"age": Integer(30), "is_active": Boolean(true), "name": String("Alice")})'
```

Setting a List

```sh
Enter command:
set numbers [1, 2, 3, 4, 5]
Set key 'numbers' with value 'List([Integer(1), Integer(2), Integer(3), Integer(4), Integer(5)])'
```

Setting a Set

```sh
Enter command:
set unique_items <"apple", "banana", "cherry">
Set key 'unique_items' with value 'Set({String("apple"), String("banana"), String("cherry")})'
```

Getting a Value

```sh
Enter command:
get username
Value for key 'username': 'String("Alice")'
```

Updating a Value

```sh
Enter command:
update username "Bob"
Updated key 'username' with new value 'String("Bob")'
```

Deleting a Key

```sh
Enter command:
delete age
Key 'age' deleted.
```

Listing All Keys

```sh
Enter command:
list
Keys: numbers, unique_items, user, username
```

Viewing Command History

```sh
Enter command:
history
Command History:
1: set username "Alice"
2: set age 30
3: set is_active true
4: set user {"name":"Alice","age":30,"is_active":true}
5: set numbers [1, 2, 3, 4, 5]
6: set unique_items <"apple", "banana", "cherry">
7: get username
8: update username "Bob"
9: delete age
10: list
11: history
```

Help

```sh
Enter command:
help
Available commands:
- set <key> <value>: Sets a key-value pair in the store.
- get <key>: Retrieves the value for the specified key.
- delete <key>: Removes the specified key from the store.
- update <key> <new_value>: Updates the value of an existing key.
- list: Lists all keys currently in the store.
- history: Displays the history of executed commands.
- help: Displays available commands and their descriptions.
- exit: Exits the program.
```

Exiting the Application

```sh
Enter command:
exit
Exiting...
```

### License

This project is licensed under the MIT License. See the LICENSE file for details.
