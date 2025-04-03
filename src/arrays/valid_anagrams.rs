use std::collections::HashMap;

fn is_anagram(first: &str, second: &str) -> bool {
    if first.len() != second.len() {
        return false
    }

    let mut hash_freq: HashMap<char, i64> = HashMap::new();
    let second_chars: Vec<char> = second.chars().collect();
    
    for &data in second_chars.iter() {
        *hash_freq.entry(data).or_insert(0) += 1;
    }

    let first_chars: Vec<char> = first.chars().collect();

    for &data in first_chars.iter() {
        *hash_freq.entry(data).or_insert(-1) -=1;
    }

    hash_freq
    .iter()
    .all(|(_key, &value)| value == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_anagram_simple() {
        assert!(is_anagram("anagram", "nagaram"));
        assert!(is_anagram("listen", "silent"));
        assert!(is_anagram("rail safety", "fairy tales"));
    }

    #[test]
    fn test_is_anagram_different_length() {
        assert!(!is_anagram("hello", "world"));
        assert!(!is_anagram("rust", "rustlang"));
        assert!(!is_anagram("a", "aa"));
    }

    #[test]
    fn test_is_anagram_same_length_not_anagram() {
        assert!(!is_anagram("hello", "world"));
        assert!(!is_anagram("rust", "dust"));
        assert!(!is_anagram("test", "best"));
    }

    #[test]
    fn test_is_anagram_case_sensitive() {
        assert!(!is_anagram("A", "a"));
        assert!(!is_anagram("Tea", "Eat"));
    }

    #[test]
    fn test_is_anagram_with_spaces() {
        assert!(is_anagram("rail safety", "fairy tales"));
        assert!(is_anagram("astronomer", "moon starer"));
        assert!(!is_anagram("hello world", "world hello "));
    }

    #[test]
    fn test_is_anagram_with_punctuation() {
        assert!(is_anagram("I am Lord Voldemort!", "Tom Marvolo Riddle!"));
        assert!(!is_anagram("hello!", "hello"));
    }

    #[test]
    fn test_is_anagram_empty_strings() {
        assert!(is_anagram("", ""));
    }

    #[test]
    fn test_is_anagram_single_character() {
        assert!(is_anagram("a", "a"));
        assert!(!is_anagram("a", "b"));
    }

    #[test]
    fn test_is_anagram_repeated_characters() {
        assert!(is_anagram("aaa", "aaa"));
        assert!(!is_anagram("aaa", "aab"));
    }

    #[test]
    fn test_is_anagram_unicode() {
        assert!(is_anagram("résumé", "ésuméré"));
        assert!(is_anagram("您好", "好您"));
        assert!(!is_anagram("café", "cafe"));
    }
}