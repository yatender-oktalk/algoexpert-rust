fn missing_element(arr: Vec<i64>) -> i64 {
    let n = 1 + arr.len();
    let expected_sum: i64 = ((n * (n + 1))/2) as i64;
    let actual_sum = arr.iter().sum::<i64>();
    return expected_sum - actual_sum
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