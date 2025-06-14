# Rust To-Do List CLI Application
A simple command-line interface (CLI) application for managing your daily tasks, built with Rust. This application allows you to add, list, complete, remove, and clear tasks, with all data persistently stored in a JSON file.


## Technical Highlights
- Structs: Tasks are represented by a Task struct containing id, description, and completed fields.

- Vectors (Vec): Tasks are managed in memory using a Vec<Task>.

- Pattern Matching (match): Used for command-line argument parsing and error handling.

- File I/O: Tasks are loaded from and saved to a tasks.json file.

- Serialization/Deserialization: The serde crate, along with serde_json, is used to effortlessly convert Rust Task structs to JSON strings and vice-versa.


## Prerequisites
To compile and run this application, you need to have Rust and Cargo (Rust's package manager and build system) installed on your system.

- Install Rust: Visit the official Rust website for installation instructions: https://www.rust-lang.org/tools/install

## Setup and Installation
- Create a new Rust project:
  Open your terminal or command prompt and run:

  ```text
  cargo new todo_cli
  cd todo_cli
  ```

- Add Dependencies:
  Open the Cargo.toml file in your todo_cli directory and add the following lines under the [dependencies] section:  
  ```text
  [dependencies]
    serde = { version = "1.0", features = ["derive"] }
    serde_json = "1.0"
  ```

  This adds the serde and serde_json crates, which are essential for handling JSON data.

- Place the Code:
  Replace the entire content of src/main.rs with the Rust code provided in the rust-todo-cli-app immersive above.      

## How to Run
After setting up the project and placing the code, you can build and run the application from your terminal.

- Build the application:  

  ```text
  cargo build --release
  ```

  This command compiles your application and creates an executable in the target/release/ directory. --release builds an optimized executable.

- Run the application:
    You can run the application directly using cargo run or by executing the compiled binary. For consistent command-line usage like todo add, it's common to add the target/release directory to your system's PATH, or copy the compiled binary (todo_cli on Linux/macOS, todo_cli.exe on Windows) to a directory already in your PATH.

    For simplicity during development, you can use cargo run -- <command> [arguments]:  

    ```text
    cargo run -- help
    ```

   Or, once built, navigate to target/release/ and run:
   ```text
   ./todo_cli help # On Linux/macOS
    # Or on Windows:
    # .\todo_cli.exe help
    ```

   For the rest of this README, we'll assume todo is accessible in your PATH. If not, prefix commands with cargo run --  or the full path to the executable.

## Usage
Here are the commands you can use with the To-Do List CLI application:       

- todo add <description> - Add a new task
  Adds a new task to your list. The description can contain spaces if enclosed in quotes.

    ```text
    cargo run -- add "Buy groceries"
    cargo run -- add "Prepare presentation for meeting"
    ```  
- todo list - List all tasks
Displays all tasks currently in your to-do list, showing their ID, completion status ([ ] for incomplete, [X] for complete), and description.    

```text
cargo run -- list
```

## Example Output:

```text
--- Your To-Do List ---
[ ] 1. Buy groceries
[ ] 2. Prepare presentation for meeting
-----------------------
```

- todo complete <id> - Mark a task as completed
  Marks the task with the specified ID as completed.

  ```text
  cargo run -- complete 1
  ```

- todo remove <id> - Remove a task
  Deletes the task with the specified ID from your to-do list.  
  ```text
  cargo run -- remove 2
  ```

- todo clear - Clear all tasks
  Prompts for confirmation and then clears all tasks from your list.  
  ```text
  todo clear
  ```

- todo help - Show help message
  Displays the usage instructions and all available commands.  
  ```text
  cargo run -- help
  ```

## File Structure
The application will automatically create and manage a tasks.json file in the same directory where you run the executable.  

```text
├── Cargo.toml
├── Cargo.lock
├── src/
│   └── main.rs
└── tasks.json  (automatically created and managed)
└── target/     (compiled binaries)
```

## How It Works
- Task Struct: Defines the structure of each to-do item. #[derive(Serialize, Deserialize)] uses serde to automatically generate code for converting Task instances to and from JSON.

- TASKS_FILE: A constant defines the name of the JSON file used for storage (tasks.json).

- load_tasks() & save_tasks():

  - load_tasks() reads the tasks.json file, parses its content as JSON, and converts it into a Vec<Task>. It handles cases where the file doesn't exist (returns an empty list) or contains invalid JSON.

  - save_tasks() takes a Vec<Task>, serializes it into a pretty-printed JSON string, and writes it back to tasks.json.

- Command Functions: Each command (add_task, list_tasks, complete_task, remove_task, clear_tasks, print_help) encapsulates the logic for that specific operation, interacting with the load_tasks and save_tasks functions to manage the persistent data.

- main() Function:

  - env::args().collect() gathers all command-line arguments.

  - A match statement is used to check the first argument (the command) and dispatches to the appropriate function. It includes basic error checking for missing arguments or invalid IDs.