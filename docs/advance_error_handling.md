# Advanced Error Handling in Rust

## Table of Contents
- [errors](errors.md)
- [Error Mapping](#error-mapping)
- [Error Handling Libraries](#error-handling-libraries)
  - [anyhow](#anyhow)
  - [thiserror](#thiserror)
  - [eyre](#eyre)

## Error Mapping

### Using map_err()
```rust
fn parse_age(input: &str) -> Result<u8, String> {
    input
        .parse::<u8>()
        .map_err(|e| format!("Failed to parse age: {}", e))
}

// Chaining map_err
fn process_age(input: &str) -> Result<u8, String> {
    input
        .trim()
        .parse::<u8>()
        .map_err(|e| format!("Invalid age: {}", e))
        .and_then(|age| {
            if age > 0 && age < 120 {
                Ok(age)
            } else {
                Err("Age must be between 1 and 120".to_string())
            }
        })
}
```

### Using map() and map_err()
```rust
#[derive(Debug)]
struct User {
    age: u8,
    name: String,
}

fn create_user(age_str: &str, name: &str) -> Result<User, String> {
    age_str
        .parse::<u8>()
        .map_err(|e| format!("Invalid age: {}", e))
        .map(|age| User {
            age,
            name: name.to_string(),
        })
}
```

## Error Handling Libraries

### anyhow
anyhow is great for applications where you don't need to define custom error types.

```rust
use anyhow::{Context, Result, anyhow};

// Simple example
fn read_config(path: &str) -> Result<Config> {
    let content = std::fs::read_to_string(path)
        .context("failed to read config file")?;
    
    let config: Config = serde_json::from_str(&content)
        .context("failed to parse config file")?;
    
    Ok(config)
}

// Creating custom errors
fn validate_user(user: &User) -> Result<()> {
    if user.age < 18 {
        return Err(anyhow!("User must be 18 or older"));
    }
    Ok(())
}

// Chaining errors
fn process_user_data(path: &str) -> Result<User> {
    let file_content = std::fs::read_to_string(path)
        .context("failed to read user file")?;
    
    let user: User = serde_json::from_str(&file_content)
        .context("failed to parse user data")?;
    
    validate_user(&user)?;
    
    Ok(user)
}
```

### thiserror
thiserror is ideal for libraries where you want to define structured error types.

```rust
use thiserror::Error;

#[derive(Error, Debug)]
enum DatabaseError {
    #[error("connection error: {0}")]
    ConnectionError(String),
    
    #[error("query failed: {msg}")]
    QueryError {
        msg: String,
        #[source]
        source: SqlError,
    },
    
    #[error("data not found")]
    NotFound,
}

// Usage with custom error type
struct Database {
    connection: Connection,
}

impl Database {
    fn query(&self, sql: &str) -> Result<Data, DatabaseError> {
        self.connection
            .execute(sql)
            .map_err(|e| DatabaseError::QueryError {
                msg: "failed to execute query".to_string(),
                source: e,
            })?;
        
        Ok(Data {})
    }
}
```

### Combining anyhow and thiserror
```rust
use thiserror::Error;
use anyhow::{Result, Context};

#[derive(Error, Debug)]
enum ServiceError {
    #[error("database error: {0}")]
    Database(#[from] DatabaseError),
    
    #[error("validation error: {0}")]
    Validation(String),
}

struct Service {
    db: Database,
}

impl Service {
    // Library function using specific error type
    fn internal_process(&self) -> Result<(), ServiceError> {
        // ... processing ...
        Ok(())
    }
    
    // Public API using anyhow
    pub fn process(&self) -> Result<()> {
        self.internal_process()
            .context("service processing failed")?;
        Ok(())
    }
}
```

### eyre
eyre is an alternative to anyhow with customizable error reporting.

```rust
use eyre::{Result, WrapErr, eyre};
use color_eyre::Report;

fn main() -> Result<(), Report> {
    // Install custom error handler
    color_eyre::install()?;
    
    let content = std::fs::read_to_string("config.json")
        .wrap_err("failed to read config")?;
    
    let config: Config = serde_json::from_str(&content)
        .wrap_err("failed to parse config")?;
    
    process_config(&config)
        .wrap_err("failed to process config")?;
    
    Ok(())
}
```

### Error Handling Best Practices

1. **Library vs Application**
   - Libraries: Use `thiserror` for specific, well-defined errors
   - Applications: Use `anyhow` for flexible error handling

2. **Error Context**
```rust
use anyhow::{Context, Result};

fn process_file(path: &str) -> Result<()> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("failed to read file: {}", path))?;
    
    process_content(&content)
        .with_context(|| "failed to process file content")?;
    
    Ok(())
}
```

3. **Combining Multiple Error Types**
```rust
use thiserror::Error;
use std::io;

#[derive(Error, Debug)]
enum AppError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    
    #[error("Parse error: {0}")]
    Parse(#[from] serde_json::Error),
    
    #[error("Validation error: {0}")]
    Validation(String),
}

// Usage with error conversion
fn process() -> Result<(), AppError> {
    let file = std::fs::File::open("data.json")?; // Converts io::Error to AppError
    let data: Value = serde_json::from_reader(file)?; // Converts serde_json::Error to AppError
    
    if !validate(&data) {
        return Err(AppError::Validation("invalid data".to_string()));
    }
    
    Ok(())
}
```

4. **Using Error Mapping for Clean APIs**
```rust
use anyhow::{Result, Context};

fn api_function() -> Result<User> {
    detailed_function()
        .map_err(|e| anyhow!("API error: {}", e))
        .context("failed to process API request")?
}
```

Remember:
- Use `anyhow` for applications where error details are logged or displayed to users
- Use `thiserror` for libraries where error handling needs to be precise
- Use `eyre` when you need customizable error reporting
- Always provide context with your errors using `.context()` or `.with_context()`
- Consider the end-user when designing error messages
