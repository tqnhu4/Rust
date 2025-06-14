// src/main.rs

// Import necessary external crates for serialization/deserialization
extern crate serde;
extern crate serde_json;

// Use macros from serde for automatic trait implementation
use serde::{Deserialize, Serialize};
// Use modules for file system operations and environment variables
use std::fs;
use std::io::{self, Write}; // io::Write for flushing stdout
use std::env; // For reading command line arguments

// Define the Task struct.
// #[derive(Serialize, Deserialize)] allows us to easily convert Task objects
// to/from JSON strings.
#[derive(Debug, Serialize, Deserialize)]
struct Task {
    id: u32,
    description: String,
    completed: bool,
}

// Define the file name where tasks will be stored.
const TASKS_FILE: &str = "tasks.json";

// Function to load tasks from the JSON file.
// It returns a Result, which can be either a Vec<Task> on success,
// or an io::Error if something goes wrong (e.g., file not found, malformed JSON).
fn load_tasks() -> Result<Vec<Task>, io::Error> {
    // Read the entire content of the file into a string.
    let data = fs::read_to_string(TASKS_FILE)?; // The '?' operator propagates errors.

    // Try to deserialize the JSON string into a Vec<Task>.
    // If successful, return the vector. If not, map the error to an io::Error.
    serde_json::from_str(&data).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

// Function to save tasks to the JSON file.
// It takes a reference to a Vec<Task> and writes it to the file.
// Returns an empty Result on success, or an io::Error on failure.
fn save_tasks(tasks: &Vec<Task>) -> Result<(), io::Error> {
    // Serialize the Vec<Task> into a pretty-printed JSON string.
    let data = serde_json::to_string_pretty(tasks)?;

    // Write the JSON string to the file.
    fs::write(TASKS_FILE, data)?;
    Ok(()) // Return Ok to indicate success
}

// Function to get the next available ID for a new task.
// It iterates through existing tasks and finds the maximum ID, then increments it.
fn get_next_id(tasks: &Vec<Task>) -> u32 {
    tasks.iter().map(|task| task.id).max().unwrap_or(0) + 1
}

// Command: Add a new task
fn add_task(description: String) {
    // Attempt to load existing tasks. If the file doesn't exist, start with an empty vector.
    let mut tasks = load_tasks().unwrap_or_else(|e| {
        if e.kind() == io::ErrorKind::NotFound {
            println!("No existing tasks file found. Creating a new one.");
        } else {
            eprintln!("Error loading tasks: {}", e);
        }
        Vec::new()
    });

    // Get a unique ID for the new task.
    let id = get_next_id(&tasks);
    // Create a new Task instance.
    let new_task = Task {
        id,
        description,
        completed: false,
    };
    // Add the new task to the vector.
    tasks.push(new_task);

    // Save the updated list of tasks.
    match save_tasks(&tasks) {
        Ok(_) => println!("Task #{} added.", id),
        Err(e) => eprintln!("Error saving tasks: {}", e),
    }
}

// Command: List all tasks
fn list_tasks() {
    // Load tasks, or print an error and return if unable to load.
    let tasks = match load_tasks() {
        Ok(t) => t,
        Err(e) => {
            if e.kind() == io::ErrorKind::NotFound {
                println!("No tasks found. Add one with 'add <description>'.");
            } else {
                eprintln!("Error loading tasks: {}", e);
            }
            return;
        }
    };

    if tasks.is_empty() {
        println!("No tasks found. Add one with 'add <description>'.");
        return;
    }

    println!("--- Your To-Do List ---");
    // Iterate through tasks and print their details.
    for task in tasks {
        let status = if task.completed { "[X]" } else { "[ ]" };
        println!("{} {}. {}", status, task.id, task.description);
    }
    println!("-----------------------");
}

// Command: Mark a task as complete
fn complete_task(id_to_complete: u32) {
    let mut tasks = match load_tasks() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Error loading tasks: {}", e);
            return;
        }
    };

    let mut found = false;
    // Iterate through tasks and find the one with the matching ID.
    for task in &mut tasks {
        if task.id == id_to_complete {
            task.completed = true;
            found = true;
            println!("Task #{} marked as complete.", id_to_complete);
            break; // Exit loop once task is found and updated.
        }
    }

    if !found {
        println!("Error: Task #{} not found.", id_to_complete);
    } else {
        // Save changes if a task was updated.
        match save_tasks(&tasks) {
            Ok(_) => {}, // Do nothing on success, message already printed
            Err(e) => eprintln!("Error saving tasks: {}", e),
        }
    }
}

// Command: Remove a task
fn remove_task(id_to_remove: u32) {
    let mut tasks = match load_tasks() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Error loading tasks: {}", e);
            return;
        }
    };

    // Filter out the task with the specified ID.
    let initial_len = tasks.len();
    tasks.retain(|task| task.id != id_to_remove); // Keep tasks whose ID is NOT the one to remove

    if tasks.len() == initial_len { // If length hasn't changed, task wasn't found
        println!("Error: Task #{} not found.", id_to_remove);
    } else {
        println!("Task #{} removed.", id_to_remove);
        // Save changes if a task was removed.
        match save_tasks(&tasks) {
            Ok(_) => {},
            Err(e) => eprintln!("Error saving tasks: {}", e),
        }
    }
}

// Command: Clear all tasks
fn clear_tasks() {
    print!("Are you sure you want to clear all tasks? (yes/no): ");
    // Ensure the prompt is displayed before reading input
    io::stdout().flush().expect("Could not flush stdout");
    
    let mut confirmation = String::new();
    io::stdin().read_line(&mut confirmation).expect("Failed to read line");
    
    if confirmation.trim().to_lowercase() == "yes" {
        // Save an empty vector, effectively clearing all tasks.
        match save_tasks(&Vec::new()) {
            Ok(_) => println!("All tasks cleared."),
            Err(e) => eprintln!("Error clearing tasks: {}", e),
        }
    } else {
        println!("Operation cancelled.");
    }
}

// Function to display help message
fn print_help() {
    println!("Usage: todo <command> [arguments]");
    println!("");
    println!("Commands:");
    println!("  add <description>     Add a new task.");
    println!("  list                  List all tasks.");
    println!("  complete <id>         Mark a task as completed by its ID.");
    println!("  remove <id>           Remove a task by its ID.");
    println!("  clear                 Clear all tasks.");
    println!("  help                  Show this help message.");
    println!("");
    println!("Examples:");
    println!("  todo add \"Buy groceries\"");
    println!("  todo list");
    println!("  todo complete 1");
    println!("  todo remove 2");
    println!("  todo clear");
}


// Main function: Entry point of the application.
fn main() {
    // Collect command line arguments into a vector of strings.
    // The first argument is always the program name itself.
    let args: Vec<String> = env::args().collect();

    // Match the command (the second argument, if it exists).
    // Use an 'if-let' binding or similar for cleaner arg parsing if more complex
    match args.get(1).map(|s| s.as_str()) {
        Some("add") => {
            // Check if a description is provided for the 'add' command.
            if args.len() > 2 {
                // Join all subsequent arguments to form the full description.
                let description = args[2..].join(" ");
                add_task(description);
            } else {
                eprintln!("Error: 'add' command requires a description.");
                print_help();
            }
        },
        Some("list") => list_tasks(),
        Some("complete") => {
            // Check if an ID is provided for the 'complete' command.
            if let Some(id_str) = args.get(2) {
                // Try to parse the ID string into a u32.
                match id_str.parse::<u32>() {
                    Ok(id) => complete_task(id),
                    Err(_) => {
                        eprintln!("Error: Invalid task ID. Please provide a number.");
                        print_help();
                    }
                }
            } else {
                eprintln!("Error: 'complete' command requires a task ID.");
                print_help();
            }
        },
        Some("remove") => {
            // Check if an ID is provided for the 'remove' command.
            if let Some(id_str) = args.get(2) {
                // Try to parse the ID string into a u32.
                match id_str.parse::<u32>() {
                    Ok(id) => remove_task(id),
                    Err(_) => {
                        eprintln!("Error: Invalid task ID. Please provide a number.");
                        print_help();
                    }
                }
            } else {
                eprintln!("Error: 'remove' command requires a task ID.");
                print_help();
            }
        },
        Some("clear") => clear_tasks(),
        Some("help") => print_help(),
        // If no command or an unknown command is provided, show help.
        _ => {
            eprintln!("Error: No command or unknown command provided.");
            print_help();
        }
    }
}
