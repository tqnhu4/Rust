# rust-tutorials
# 🦀 7-Day Rust Learning Roadmap for Beginners

Welcome to your accelerated Rust learning journey! This roadmap is meticulously crafted to guide absolute beginners through the fundamentals of Rust within seven days, emphasizing practical application and rapid skill acquisition.

Each day focuses on a distinct set of core Rust concepts, complete with illustrative examples and a culminating practical project to solidify your understanding.

## 📅 Day 1: Rust Basics & Setup
Today is all about getting Rust set up and writing your very first program, while also understanding its fundamental concepts.

### Content:
- **Introduction to Rust:** What Rust is, its key promises (performance, reliability, productivity), and its growing popularity.
- **Installation & Environment Setup:**
  - Detailed steps to install Rust using rustup on your operating system (Windows, macOS, Linux).
  - Setting up a suitable IDE (e.g., VS Code with the rust-analyzer extension).
  - Using Cargo: Rust's build system and package manager.
  - cargo new: Create a new project.
  - cargo run: Compile and run a project.
  - cargo build: Compile a project.
- **Your First Rust Program:** Writing and executing a "Hello, World!" program.
- **Basic Syntax & Structure:**
  - Understanding fn main(), println!, and macros.
  - Comments (// for single-line, /* */ for multi-line).
- **Variables & Basic Data Types:**
  - Declaring variables (let, mut).
  - Common primitive data types: Integers (i32, u64), Floating-point numbers (f64), Booleans (bool), Characters (char).
  - Type inference and explicit type annotations.
### Example:

```rust
// This is a single-line comment

/*
This is a multi-line comment.
Welcome to Rust!
*/

// The `main` function is the entry point of the program.
fn main() {
    // Your first Rust program
    // `println!` is a macro used for printing text to the console.
    println!("Hello, Rust World!");

    // Variable declaration: `let` creates an immutable variable by default.
    let favorite_language = "Rust";
    // favorite_language = "Go"; // This would cause a compile-time error because `favorite_language` is immutable.

    // To make a variable mutable, use `mut`.
    let mut age = 30;
    println!("I am initially {} years old.", age);
    age = 31; // Now we can change its value.
    println!("Now I am {} years old.", age);

    // Explicit type annotation
    let height: f64 = 1.75;
    let is_awesome: bool = true;
    let initial: char = 'R';

    println!("My height is {} meters.", height);
    println!("Is Rust awesome? {}", is_awesome);
    println!("My initial is: {}", initial);

    // Rust can infer types
    let inferred_number = 42; // Inferred as `i32`
    println!("Inferred number: {}", inferred_number);
}
```

----

## 📅 Day 2: Control Flow & Compound Data Types
Today, you'll learn how to control the execution flow of your Rust programs using conditional statements and loops, and how to group data together.

### Content:
- **Operators:**
  - Arithmetic Operators: +, -, *, /, %.
  - Comparison Operators: ==, !=, >, <, >=, <=.
  - Logical Operators: && (AND), || (OR), ! (NOT).
- **Conditional Statements (if, else if, else):** Decision-making based on conditions.
  - if is an expression (it can return a value).
- **Looping Constructs:**
  - loop: An infinite loop that you explicitly break from.
  - while: Repeats a block of code while a condition is true.
  - for: Iterating over elements in a collection (often using ranges).
- **Compound Data Types:**
  - Tuples: Fixed-size, ordered collections of different types.
  - Arrays: Fixed-size, ordered collections of elements of the same type.
### Example:

```rust
fn main() {
    // --- Operators ---
    let x = 10;
    let y = 3;
    println!("\n--- Operators ---");
    println!("{} + {} = {}", x, y, x + y);
    println!("{} - {} = {}", x, y, x - y);
    println!("{} * {} = {}", x, y, x * y);
    println!("{} / {} = {}", x, y, x / y); // Integer division
    println!("{} % {} = {}", x, y, x % y);

    let is_true = true;
    let is_false = false;
    println!("Is true AND false? {}", is_true && is_false);
    println!("Is true OR false? {}", is_true || is_false);
    println!("NOT true? {}", !is_true);


    // --- Conditional Statements ---
    let score = 75;
    println!("\n--- Conditional Statements ---");
    if score >= 90 {
        println!("Excellent!");
    } else if score >= 70 {
        println!("Good!");
    } else {
        println!("Needs improvement.");
    }

    // `if` as an expression
    let status = if score >= 60 { "Passed" } else { "Failed" };
    println!("Status: {}", status);


    // --- Looping Constructs ---
    println!("\n--- Looping Constructs ---");

    // `loop` (infinite loop with break)
    let mut counter = 0;
    loop {
        println!("Loop count: {}", counter);
        counter += 1;
        if counter == 3 {
            break; // Exit the loop
        }
    }

    // `while` loop
    let mut num = 0;
    while num < 3 {
        println!("While count: {}", num);
        num += 1;
    }

    // `for` loop (iterating over a range)
    for i in 0..3 { // Exclusive range (0, 1, 2)
        println!("For loop (range): {}", i);
    }

    // --- Compound Data Types ---
    println!("\n--- Compound Data Types ---");

    // Tuple
    let person_data: (String, i32, f64) = ("Alice".to_string(), 30, 1.65);
    println!("Name: {}", person_data.0);
    println!("Age: {}", person_data.1);
    println!("Height: {}", person_data.2);

    // Destructuring a tuple
    let (name, age_from_tuple, height_from_tuple) = person_data;
    println!("Destructured Name: {}", name);

    // Array
    let numbers: [i32; 5] = [1, 2, 3, 4, 5]; // Array of 5 i32 integers
    println!("Array: {:?}", numbers); // Use `{:?}` for debug printing
    println!("First element of array: {}", numbers[0]);

    // Accessing elements that don't exist in arrays will panic at runtime
    // println!("Sixth element: {}", numbers[5]); // This would panic!
}
```


## 📅 Day 3: Functions & Ownership
Today you'll learn to modularize your code with functions and grasp Rust's most unique and powerful concept: Ownership.

### Content:
- **Functions:**
  - Defining and calling functions (fn).
  - Parameters and return values (explicit -> Type).
  - Statements vs. Expressions (functions can return values implicitly from their last expression).
- **Ownership:** Rust's core memory management model.
  - **Rules of Ownership:**
    - Each value in Rust has a variable that's called its owner.
    - There can only be one owner at a time.
    - When the owner goes out of scope, the value will be dropped.
  - **Move Semantics:** How ownership is transferred.
  - **Borrowing (References):** How to allow multiple parts of your code to access data without taking ownership.
    - Immutable references (&).
    - Mutable references (&mut).
  - **Dangling References:** Understanding why Rust prevents them.
### Example:

```rust
// --- Functions ---
// A simple function that takes no arguments and returns nothing
fn greet() {
    println!("Hello from a function!");
}

// Function with parameters and an explicit return type
fn add_numbers(x: i32, y: i32) -> i32 {
    x + y // This is an expression, its value is implicitly returned
}

// Function returning multiple values using a tuple
fn get_user_info() -> (String, i32) {
    ("Jane Doe".to_string(), 28)
}

// --- Ownership Examples ---

// Function that takes ownership of a String
fn takes_ownership(some_string: String) {
    println!("Inside takes_ownership: {}", some_string);
} // `some_string` goes out of scope and `drop` is called.

// Function that takes a reference to a String (borrows)
fn takes_reference(some_string: &String) {
    println!("Inside takes_reference: {}", some_string);
} // `some_string` goes out of scope, but the value it points to is NOT dropped.

// Function that takes a mutable reference to a String
fn changes_string(some_string: &mut String) {
    some_string.push_str(", and more!");
    println!("Inside changes_string: {}", some_string);
}

fn main() {
    println!("\n--- Functions ---");
    greet(); // Call the greet function

    let sum = add_numbers(5, 7);
    println!("Sum of 5 and 7: {}", sum);

    let (user_name, user_age) = get_user_info();
    println!("User: {} is {} years old.", user_name, user_age);


    println!("\n--- Ownership ---");
    let s1 = String::from("hello"); // s1 owns "hello"
    takes_ownership(s1); // s1's value is moved into the function.
                         // s1 is no longer valid here.
    // println!("{}", s1); // This would be a compile-time error!

    let s2 = String::from("world"); // s2 owns "world"
    takes_reference(&s2); // s2's value is borrowed by the function.
                          // s2 is still valid here.
    println!("s2 after takes_reference: {}", s2);


    let mut s3 = String::from("mutable string"); // s3 owns "mutable string"
    changes_string(&mut s3); // s3's value is mutably borrowed.
    println!("s3 after changes_string: {}", s3);

    // Rule: You can have EITHER one mutable reference OR any number of immutable references
    // at any given time, but not both simultaneously.
    let s4 = String::from("multiple borrows");
    let r1 = &s4; // first immutable reference
    let r2 = &s4; // second immutable reference
    println!("r1: {}, r2: {}", r1, r2); // s4 is valid here

    // let r3 = &mut s4; // This would cause a compile-time error because r1 and r2 are still in scope!
}
```

---

## 📅 Day 4: Structs, Enums & Pattern Matching
Today, you'll learn to create custom data types and powerful ways to handle different data variants.

### Content:
- **Structs:** Custom data types for grouping related fields.
  - Defining structs (struct).
  - Creating instances and accessing fields.
  - Tuple structs (like named tuples).
  - Unit-like structs.
  - Methods: Functions associated with a specific struct.
  - impl block.
  - &self (immutable reference to self) and &mut self (mutable reference to self).
  - Associated functions (static methods, no self, e.g., String::from).
- **Enums (Enumerations):** Custom types that can be one of a few defined variants.
  - Defining enums (enum).
  - Enums with data.
- **match Expression (Pattern Matching):**
  - A powerful control flow operator for handling different enum variants or other patterns.
  - Exhaustiveness check (Rust ensures you handle all possible cases).
  - _ (wildcard pattern) for catching all other cases.
- **Option<T> Enum:** Rust's way of representing optional values (absence of a value).
  - Some(T) vs. None.
  - Handling Option with match or if let.
### Example:

```rust
// --- Structs ---
// Define a struct named `User`
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// Define a method for the `User` struct using an `impl` block
impl User {
    // A method with an immutable reference to self
    fn greet(&self) {
        println!("Hello, {}! Your email is {}.", self.username, self.email);
    }

    // A method with a mutable reference to self
    fn increment_sign_in_count(&mut self) {
        self.sign_in_count += 1;
    }

    // An associated function (constructor-like)
    fn build_user(username: String, email: String) -> User {
        User {
            username, // Shorthand for `username: username`
            email,
            active: true,
            sign_in_count: 1,
        }
    }
}

// --- Enums ---
// Define an enum named `Message`
enum Message {
    Quit,
    Move { x: i32, y: i32 }, // Enum variant with named fields
    Write(String),           // Enum variant with a String
    ChangeColor(i32, i32, i32), // Enum variant with a tuple of i32s
}

fn main() {
    println!("\n--- Structs ---");
    // Create a struct instance
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someuser123"),
        active: true,
        sign_in_count: 1,
    };

    println!("User1's email: {}", user1.email);
    user1.email = String::from("another@example.com"); // Change a field
    println!("User1's new email: {}", user1.email);

    user1.greet(); // Call a method
    user1.increment_sign_in_count();
    println!("User1's sign-in count: {}", user1.sign_in_count);

    // Use an associated function
    let user2 = User::build_user(String::from("newuser"), String::from("new@example.com"));
    println!("User2's username: {}", user2.username);


    println!("\n--- Enums & Match ---");
    let msg = Message::Write(String::from("hello world"));
    let quit_msg = Message::Quit;
    let move_msg = Message::Move { x: 10, y: 20 };

    // Use `match` to handle different enum variants
    fn handle_message(msg: Message) {
        match msg {
            Message::Quit => {
                println!("The Quit variant has no data.");
            }
            Message::Move { x, y } => {
                println!("Move to x: {}, y: {}", x, y);
            }
            Message::Write(text) => {
                println!("Text message: {}", text);
            }
            Message::ChangeColor(r, g, b) => {
                println!("Change the color to R: {}, G: {}, B: {}", r, g, b);
            }
        }
    }

    handle_message(msg);
    handle_message(quit_msg);
    handle_message(move_msg);
    handle_message(Message::ChangeColor(255, 0, 100));


    println!("\n--- Option<T> Enum ---");
    // `Option<T>` is a standard library enum
    let some_number = Some(5);
    let no_number: Option<i32> = None;

    // Handling Option with `match`
    match some_number {
        Some(value) => println!("Got a number: {}", value),
        None => println!("Didn't get a number."),
    }

    match no_number {
        Some(value) => println!("Got a number: {}", value),
        None => println!("Didn't get a number."),
    }

    // Handling Option with `if let` (shorter way for specific cases)
    if let Some(value) = some_number {
        println!("Using if let, got: {}", value);
    } else {
        println!("Using if let, got None.");
    }
}
```

---

## 📅 Day 5: Error Handling & Vectors
Today, you'll learn about Rust's robust error handling mechanisms and its dynamic array type.

### Content:
- **Error Handling:**
  - Result<T, E> Enum: Rust's primary way of representing operations that can succeed or fail.
    - Ok(T) for success.
    - Err(E) for failure.
  - Handling Result with match.
  - panic!: For unrecoverable errors (crashes the program).
  - The ? (question mark) operator: A concise way to propagate errors.
- **Vectors (Vec<T>):** Growable, heap-allocated lists of values of the same type.
  - Creating vectors (Vec::new(), vec![] macro).
  - Adding elements (push).
  - Accessing elements (by index, get() method).
  - Iterating over vectors.
### Example:

```rust
use std::fs::File; // Import the File type from the standard library
use std::io::{self, Read}; // Import io module and Read trait

// --- Error Handling ---

// Function that might return an error
fn read_username_from_file(filepath: &str) -> Result<String, io::Error> {
    // `File::open` returns a `Result<File, io::Error>`
    let mut f = File::open(filepath)?; // The `?` operator propagates the error if one occurs

    let mut s = String::new();
    f.read_to_string(&mut s)?; // The `?` operator also works here
    Ok(s) // Return the string on success
}

fn main() {
    println!("\n--- Error Handling ---");

    // Try reading a file that doesn't exist
    let username_result = read_username_from_file("nonexistent.txt");
    match username_result {
        Ok(username) => println!("Username: {}", username),
        Err(e) => println!("Error reading username: {}", e),
    }

    // Create a dummy file for successful read
    let _ = File::create("user.txt")
        .expect("Failed to create user.txt");
    let _ = std::fs::write("user.txt", "rust_user")
        .expect("Failed to write to user.txt");

    // Try reading the created file
    let username_result = read_username_from_file("user.txt");
    match username_result {
        Ok(username) => println!("Successfully read username: {}", username),
        Err(e) => println!("Error reading username: {}", e),
    }

    // `panic!` example (usually for unrecoverable errors or during development)
    // panic!("Crash and burn!"); // Uncommenting this will stop the program immediately


    // --- Vectors ---
    println!("\n--- Vectors ---");

    // Create an empty vector and add elements
    let mut numbers: Vec<i32> = Vec::new();
    numbers.push(10);
    numbers.push(20);
    numbers.push(30);
    println!("Vector after pushes: {:?}", numbers);

    // Create a vector with initial values using the `vec!` macro
    let colors = vec!["red", "green", "blue"];
    println!("Colors vector: {:?}", colors);

    // Access elements by index (returns `Option<T>`)
    let first_color = colors.get(0);
    match first_color {
        Some(color) => println!("First color: {}", color),
        None => println!("No first color."),
    }

    let fourth_color = colors.get(3); // This index is out of bounds
    match fourth_color {
        Some(color) => println!("Fourth color: {}", color),
        None => println!("No fourth color (index out of bounds)."),
    }

    // Direct access (will panic if index is out of bounds)
    // println!("Direct access to fourth color: {}", colors[3]); // This would panic!

    // Iterate over a vector
    println!("Iterating over numbers:");
    for n in &numbers { // `&numbers` borrows the vector, so we don't take ownership
        println!("- {}", n);
    }

    // Iterate and modify (requires mutable borrow)
    println!("Iterating and modifying numbers:");
    for n in &mut numbers {
        *n += 1; // Dereference `n` to modify the value
    }
    println!("Numbers after modification: {:?}", numbers);
}
```

## 📅 Day 6: Generics, Traits & Modules
Today, you'll learn about writing flexible, reusable code with generics and traits, and organizing your code with modules.

### Content:
- **Generics:** Writing code that works for multiple data types without duplication.
  - Generic functions.
  - Generic structs and enums.
- **Traits:** Defining shared behavior across different types.
  - Defining traits (trait).
  - Implementing traits for types (impl Trait for Type).
  - Trait bounds (specifying that a generic type must implement a certain trait).
- **Modules & Crates:** Organizing your Rust code.
  - **Modules (mod):** Namespacing and privacy (public/private).
  - **Crates:** A compilation unit in Rust (binary or library).
  - use keyword: Bringing paths into scope.
  - pub: Making items public.
### Example:

```rust
// --- Generics ---

// A generic function that finds the largest element in a list
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// A generic struct
struct Point<T> {
    x: T,
    y: T,
}

// --- Traits ---

// Define a trait named `Summary`
trait Summary {
    fn summarize(&self) -> String;
    // Default implementation (optional)
    fn summarize_default(&self) -> String {
        String::from("(Read more...)")
    }
}

// Implement the `Summary` trait for the `NewsArticle` struct
struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

// Implement the `Summary` trait for the `Tweet` struct
struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// Function that takes any type that implements the `Summary` trait
fn notify(item: &impl Summary) { // `impl Trait` syntax (shorthand for trait bounds)
    println!("Breaking news! {}", item.summarize());
}

// --- Modules ---
// Define a module named `garden`
mod garden {
    // Define a sub-module named `vegetables`
    pub mod vegetables { // `pub` makes the module public
        // Define a public struct inside the `vegetables` module
        pub struct Asparagus {
            pub name: String,
        }
    }
}


fn main() {
    println!("\n--- Generics ---");
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };
    println!("Integer point: ({}, {})", integer_point.x, integer_point.y);
    println!("Float point: ({}, {})", float_point.x, float_point.y);


    println!("\n--- Traits ---");
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("New tweet: {}", tweet.summarize());
    notify(&tweet);

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup!"),
        location: String::from("Pittsburgh, PA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best team in hockey."),
    };
    println!("New article: {}", article.summarize());
    println!("Default summary: {}", article.summarize_default());


    println!("\n--- Modules ---");
    // Use the `use` keyword to bring items into scope
    use crate::garden::vegetables::Asparagus; // `crate` refers to the root of the current crate

    let plant = Asparagus { name: String::from("Green Asparagus") };
    println!("I'm growing {:?}!", plant.name);
}
```

## 📅 Day 7: Practical Project: Simple CLI Calculator
Today is about consolidating your knowledge by building a small, practical Rust application.

### Content:
- **Building a Simple Command-Line Calculator:**
  - **Features:** Take two numbers and an operation (+, -, *, /) from the command line, perform the calculation, and print the result.
  - **Concepts applied:**
    - Functions
    - main function arguments (std::env::args).
    - Parsing strings to numbers (parse()).
    - Error handling (Result, match, expect).
    - Control flow (if/else if/else, match).
    - Basic I/O (println!).
### Example Project (src/main.rs in a new Cargo project):

```rust
// To run this project:
// 1. Create a new cargo project: `cargo new simple_calculator`
// 2. Navigate into the project directory: `cd simple_calculator`
// 3. Replace the content of `src/main.rs` with the code below.
// 4. Run from terminal: `cargo run 10 + 5` (or any other numbers and operation)
//    Try `cargo run 10 / 0` to see error handling.
//    Try `cargo run ten + five` to see parsing errors.

use std::env; // To access command-line arguments
use std::process; // To exit the program with an error code

fn main() {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();

    // Expecting exactly 3 arguments: program_name, num1, operation, num2
    if args.len() != 4 {
        eprintln!("Usage: {} <number1> <operator> <number2>", args[0]);
        eprintln!("Operators: +, -, *, /");
        process::exit(1); // Exit with an error code
    }

    // Parse the numbers
    let num1: f64 = match args[1].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Error: First argument '{}' is not a valid number.", args[1]);
            process::exit(1);
        }
    };

    let operator = &args[2]; // Operator is a String slice

    let num2: f64 = match args[3].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Error: Third argument '{}' is not a valid number.", args[3]);
            process::exit(1);
        }
    };

    let result = calculate(num1, operator, num2);

    match result {
        Ok(val) => println!("Result: {}", val),
        Err(e) => eprintln!("Error: {}", e), // Print the error message
    }
}

// Function to perform the calculation
fn calculate(num1: f64, operator: &str, num2: f64) -> Result<f64, String> {
    match operator {
        "+" => Ok(num1 + num2),
        "-" => Ok(num1 - num2),
        "*" => Ok(num1 * num2),
        "/" => {
            if num2 == 0.0 {
                // Return an Err variant with a custom error message
                Err(String::from("Division by zero is not allowed."))
            } else {
                Ok(num1 / num2)
            }
        },
        _ => Err(format!("Unsupported operator: '{}'", operator)), // Handle unknown operators
    }
}
```

## 💡 Tips for Quick Learning and Immediate Application:
- **Hands-on Practice is Crucial!** The best way to learn Rust is by writing code. Implement every example, and then try to modify it or create your own variations.
- **Understand the Borrow Checker:** Rust's ownership and borrowing system can be challenging initially. Take your time to understand it; it's what makes Rust so safe and fast. Don't get discouraged by compile-time errors related to borrowing.
- **Use cargo check frequently:** This command quickly checks your code for errors without compiling the executable.
- **Read the Official Documentation:** The Rust Book is an excellent resource and widely recommended for beginners. Rust's official documentation is generally top-notch.
- **Error Messages are Your Friend:** Rust's compiler errors are often very helpful. Read them carefully; they usually point you in the right direction.
- **Community and Resources:** Engage with the Rust community on platforms like Reddit (r/rust), Discord, or Stack Overflow. There are many helpful people and resources available.