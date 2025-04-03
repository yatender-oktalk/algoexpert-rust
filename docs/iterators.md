# Rust Iterators Cheatsheet

## Core Iterator Types

```rust
// Three main ways to iterate over a collection

// 1. iter() - Borrows each element (&T)
for item in collection.iter() {
    // item is of type &T
}

// 2. iter_mut() - Mutably borrows each element (&mut T)
for item in collection.iter_mut() {
    // item is of type &mut T
    *item += 1; // Can modify values
}

// 3. into_iter() - Takes ownership (T)
for item in collection.into_iter() {
    // item is of type T
    // collection is consumed and can't be used after this
}
```

## Iteration Shortcuts

```rust
// These are shortcuts for the above methods

// Equivalent to collection.iter()
for item in &collection {
    // item is of type &T
}

// Equivalent to collection.iter_mut()
for item in &mut collection {
    // item is of type &mut T
}

// Equivalent to collection.into_iter()
for item in collection {
    // item is of type T
    // collection is consumed
}
```

## HashMap Specific Iteration

```rust
// Iterating over a HashMap

// Iterate through key-value pairs
for (key, value) in &map {
    // key: &K, value: &V
}

// Iterate just over keys
for key in map.keys() {
    // key: &K
}

// Iterate just over values
for value in map.values() {
    // value: &V
}

// Iterate over mutable values
for value in map.values_mut() {
    // value: &mut V
    *value += 10;
}
```

## Common Iterator Methods

```rust
// Transform each element
let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();

// Keep only elements that match a predicate
let evens: Vec<&i32> = numbers.iter().filter(|x| *x % 2 == 0).collect();

// Find the first element matching a condition
if let Some(first_even) = numbers.iter().find(|x| *x % 2 == 0) {
    println!("First even: {}", first_even);
}

// Fold/reduce elements into a single value
let sum = numbers.iter().fold(0, |acc, x| acc + x);

// Check if any element matches a condition
let has_even = numbers.iter().any(|x| x % 2 == 0);

// Check if all elements match a condition
let all_positive = numbers.iter().all(|x| *x > 0);

// Count elements that match a condition
let even_count = numbers.iter().filter(|x| *x % 2 == 0).count();
```

## Chaining Iterators

```rust
// Multiple operations can be chained
let processed: Vec<i32> = numbers.iter()
    .filter(|x| *x > 0)         // keep positive numbers
    .map(|x| x * 2)             // double each
    .filter(|x| *x % 10 == 0)   // keep multiples of 10
    .collect();                 // collect into a Vec

// Enumerate to get index alongside value
for (index, value) in collection.iter().enumerate() {
    println!("Item {} = {}", index, value);
}

// zip to iterate over two collections in parallel
for (a, b) in collection1.iter().zip(collection2.iter()) {
    println!("{} + {} = {}", a, b, a + b);
}
```

## Consuming vs Non-Consuming Methods

```rust
// Non-consuming methods (return another iterator)
.map()
.filter()
.take()
.skip()
.enumerate()
.zip()

// Consuming methods (produce a final result)
.collect()
.count()
.sum()
.fold()
.reduce()
.any()
.all()
.find()
```

# Rust HashMap Iteration and Chaining

## Modern HashMap Iteration

```rust
// Given a HashMap
let mut word_counts: HashMap<String, i32> = HashMap::new();
word_counts.insert("hello".to_string(), 5);
word_counts.insert("world".to_string(), 3);
word_counts.insert("rust".to_string(), 10);

// Iterating with Functional Patterns Instead of for-loops
```

## Basic Transformations

```rust
// Find all keys with values > 5
let popular_words: Vec<&String> = word_counts.iter()
    .filter(|(_, &count)| count > 5)
    .map(|(word, _)| word)
    .collect();

// Double all counts
let doubled: HashMap<String, i32> = word_counts.iter()
    .map(|(key, value)| (key.clone(), value * 2))
    .collect();

// Get sum of all values
let total_count: i32 = word_counts.values().sum();

// Get largest count
if let Some((&word, &count)) = word_counts.iter()
    .max_by_key(|(_, count)| *count) {
    println!("Most common word: {} (count: {})", word, count);
}
```

## Inspecting Keys and Values

```rust
// Print all entries
word_counts.iter()
    .for_each(|(key, value)| println!("{}: {}", key, value));

// Debug print the entire HashMap structure
println!("{:#?}", word_counts);

// Converting to a Vec for debugging
let entries: Vec<(&String, &i32)> = word_counts.iter().collect();
println!("{:?}", entries);

// Get sorted keys
let mut sorted_keys: Vec<&String> = word_counts.keys().collect();
sorted_keys.sort();
```

## Advanced Transformations

```rust
// Group by word length
let by_length: HashMap<usize, Vec<&String>> = word_counts.keys()
    .fold(HashMap::new(), |mut acc, word| {
        acc.entry(word.len()).or_default().push(word);
        acc
    });

// Find most frequent character across all keys
let char_freq: HashMap<char, usize> = word_counts.keys()
    .flat_map(|s| s.chars())
    .fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    });

// Convert to Vec<(String, i32)> sorted by count
let mut sorted_entries: Vec<(String, i32)> = word_counts
    .into_iter()  // Takes ownership!
    .collect();
sorted_entries.sort_by_key(|(_, count)| -count);  // Sort descending
```

## Working with Entry API in Chains

```rust
// Conditionally update multiple entries
["hello", "world", "rust"].iter()
    .for_each(|word| {
        word_counts.entry(word.to_string())
            .and_modify(|count| *count += 1)
            .or_insert(1);
    });

// Insert or update based on computations
["short", "medium_length", "very_long_word"].iter()
    .for_each(|word| {
        let len = word.len();
        word_counts.entry(word.to_string())
            .and_modify(|count| *count += len as i32)
            .or_insert(len as i32);
    });
```

## Pattern Matching with Iterators

```rust
// Process entries based on their values
word_counts.iter()
    .for_each(|(word, &count)| match count {
        0 => println!("{} not found", word),
        1 => println!("{} appears once", word),
        n if n < 5 => println!("{} appears {} times", word, n),
        _ => println!("{} appears frequently", word),
    });
```