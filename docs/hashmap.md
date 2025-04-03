# Rust HashMap Cheatsheet

## Setup
```rust
use std::collections::HashMap;
use std::collections::hash_map::Entry;

// Create a new HashMap
let mut map: HashMap<KeyType, ValueType> = HashMap::new();

// With initial capacity
let mut map = HashMap::with_capacity(capacity);
```

## Basic Operations

```rust
// Insert - returns Option with old value if key existed
let old_value = map.insert(key, value);

// Get - returns Option<&Value>
if let Some(value) = map.get(&key) {
    // Use value
}

// Get mutable reference
if let Some(value) = map.get_mut(&key) {
    *value += 1;
}

// Remove - returns Option with the value if found
let removed_value = map.remove(&key);

// Check if key exists
if map.contains_key(&key) {
    // Key exists
}

// Get size and check if empty
let size = map.len();
let is_empty = map.is_empty();

// Clear all entries
map.clear();
```

## Entry API (Powerful Operations)

```rust
// Insert if key doesn't exist
let value = map.entry(key).or_insert(default_value);

// Insert with function if key doesn't exist
let value = map.entry(key).or_insert_with(|| compute_value());

// Insert with default if key doesn't exist, and return a mutable reference
let value = map.entry(key).or_default();

// Most common pattern - increment counter
*map.entry(key).or_insert(0) += 1;

// Advanced entry pattern
match map.entry(key) {
    Entry::Occupied(mut entry) => {
        // Key exists, modify its value
        *entry.get_mut() += 1;
    },
    Entry::Vacant(entry) => {
        // Key doesn't exist, insert new value
        entry.insert(initial_value);
    }
}
```

## Iteration

```rust
// Iterate over references to keys and values
for (key, value) in &map {
    println!("{key}: {value}");
}

// Iterate over mutable references
for (key, value) in &mut map {
    *value *= 2;
}

// Iterate over keys only
for key in map.keys() {
    println!("Key: {key}");
}

// Iterate over values only
for value in map.values() {
    println!("Value: {value}");
}

// Iterate over mutable values
for value in map.values_mut() {
    *value += 10;
}
```

## Utility Operations

```rust
// Get value or default (without modifying map)
let count = map.get(&key).copied().unwrap_or(0);
let count = *map.get(&key).unwrap_or(&0);

// Collect from an iterator into a HashMap
let char_counts: HashMap<char, usize> = 
    "hello".chars()
           .fold(HashMap::new(), |mut acc, c| {
                *acc.entry(c).or_insert(0) += 1;
                acc
           });

// Another way to collect
let char_counts: HashMap<char, usize> = 
    "hello".chars()
           .map(|c| (c, 1))
           .collect();  // Requires that key implements Eq + Hash
```