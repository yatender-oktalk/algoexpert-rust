// Input: arr[] = [8, 2, 4, 5, 3, 7, 1]
// Output: 6
// Explanation: All the numbers from 1 to 8 are present except 6.

// Input: arr[] = [1, 2, 3, 5]
// Output: 4
// Explanation: Here the size of the array is 4, so the range will be [1, 5].
// The missing number between 1 to 5 is 4

fn missing_element(arr: Vec<i64>) -> i64 {
    let n = arr.len() + 1;
    let expected_sum: i64 = ((n * (n + 1)) / 2) as i64;
    let sum: i64 = arr.iter().sum();
    return expected_sum - sum;
}

mod tests {
    use super::*;

    #[test]
    fn test_equilibirium() {
        assert_eq!(missing_element(vec![1, 2, 3, 5]), 4);
        assert_eq!(missing_element(vec![8, 2, 4, 5, 3, 7, 1]), 6);
        assert_eq!(missing_element(vec![1]), 2);
    }
}
