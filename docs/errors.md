# Error Handling in Rust

## Table of Contents
1. [Panic vs Result](#panic-vs-result)
2. [Using Result Type](#using-result-type)
3. [Custom Error Types](#custom-error-types)
4. [Error Propagation](#error-propagation)
5. [Practical Examples](#practical-examples)

## Panic vs Result

### Panic
Use `panic!` when the program reaches an unrecoverable state.

```rust
fn divide(a: i32, b: i32) {
    if b == 0 {
        panic!("Division by zero!");
    }
    println!("{}", a / b);
}
```

### Result
Use `Result` for recoverable errors.

```rust
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Err("Division by zero!".to_string());
    }
    Ok(a / b)
}

// Usage
match divide(10, 2) {
    Ok(result) => println!("Result: {}", result),
    Err(e) => println!("Error: {}", e),
}
```

## Using Result Type

### Basic Result Pattern
```rust
enum Result<T, E> {
    Ok(T),    // Success case with value of type T
    Err(E),   // Error case with value of type E
}

// Example
fn parse_number(input: &str) -> Result<i32, std::num::ParseIntError> {
    input.parse()
}
```

### Using `?` Operator
```rust
fn process_number(input: &str) -> Result<i32, std::num::ParseIntError> {
    let num = input.parse()?;  // Returns error if parse fails
    Ok(num * 2)
}
```

## Custom Error Types

### Creating Custom Error Type
```rust
#[derive(Debug)]
enum TreeError {
    NodeNotFound,
    DuplicateValue,
    InvalidInput,
}

impl std::fmt::Display for TreeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TreeError::NodeNotFound => write!(f, "Node not found in tree"),
            TreeError::DuplicateValue => write!(f, "Value already exists in tree"),
            TreeError::InvalidInput => write!(f, "Invalid input provided"),
        }
    }
}

impl std::error::Error for TreeError {}
```

### Using Custom Error Type
```rust
struct BinaryTree<T> {
    root: Option<Box<Node<T>>>,
}

impl<T: Ord> BinaryTree<T> {
    fn insert(&mut self, value: T) -> Result<(), TreeError> {
        if self.contains(&value) {
            return Err(TreeError::DuplicateValue);
        }
        // Insert implementation
        Ok(())
    }
}
```

## Error Propagation

### Using `?` Operator for Error Propagation
```rust
fn read_and_process(filename: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(filename)?;
    let number = content.parse::<i32>()?;
    Ok(number * 2)
}
```

### Combining Different Error Types
```rust
#[derive(Debug)]
enum ApplicationError {
    IoError(std::io::Error),
    ParseError(std::num::ParseIntError),
    CustomError(String),
}

impl From<std::io::Error> for ApplicationError {
    fn from(err: std::io::Error) -> ApplicationError {
        ApplicationError::IoError(err)
    }
}

impl From<std::num::ParseIntError> for ApplicationError {
    fn from(err: std::num::ParseIntError) -> ApplicationError {
        ApplicationError::ParseError(err)
    }
}
```

## Practical Examples

### File Operations with Error Handling
```rust
use std::fs::File;
use std::io::{self, Read};

fn read_file_contents(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// Usage
fn main() {
    match read_file_contents("example.txt") {
        Ok(contents) => println!("File contents: {}", contents),
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}
```

### Binary Tree with Error Handling
```rust
#[derive(Debug)]
enum TreeError {
    NodeNotFound,
    DuplicateValue,
}

struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: Ord> Node<T> {
    fn insert(&mut self, value: T) -> Result<(), TreeError> {
        match value.cmp(&self.value) {
            std::cmp::Ordering::Equal => {
                Err(TreeError::DuplicateValue)
            }
            std::cmp::Ordering::Less => {
                match self.left {
                    None => {
                        self.left = Some(Box::new(Node {
                            value,
                            left: None,
                            right: None,
                        }));
                        Ok(())
                    }
                    Some(ref mut left) => left.insert(value),
                }
            }
            std::cmp::Ordering::Greater => {
                match self.right {
                    None => {
                        self.right = Some(Box::new(Node {
                            value,
                            left: None,
                            right: None,
                        }));
                        Ok(())
                    }
                    Some(ref mut right) => right.insert(value),
                }
            }
        }
    }
}

// Usage
fn main() {
    let mut root = Node {
        value: 5,
        left: None,
        right: None,
    };

    match root.insert(3) {
        Ok(()) => println!("Value inserted successfully"),
        Err(e) => eprintln!("Error inserting value: {:?}", e),
    }
}
```

[next](advance_error_handling.md)