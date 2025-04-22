fn find_unique(arr: Vec<i32>) -> i32 {
    let mut result = 0;
    for &x in arr.iter() {
        result ^= x;
    }

    return result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_unique() {
        assert_eq!(find_unique(vec![1, 2, 1, 5, 5]), 2);
        assert_eq!(find_unique(vec![2, 30, 2, 15, 20, 30, 15]), 20);
    }
}