use std::collections::HashMap;

pub fn two_num_sum(arr: &Vec<i64>, target: i64) -> Vec<i64> {
    let mut nums: HashMap<i64, ()> = HashMap::new();

    for &value in arr.iter() {
        let complement = target - value;
        if let true = nums.contains_key(&complement){
            return vec![complement, value]
        }
        nums.insert(value, ());
    }

    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_num_sum() {
        // Basic test
        assert_eq!(two_num_sum(&vec![2, 7, 11, 15], 9).len(), 2);
        assert!(
            vec![2, 7] == two_num_sum(&vec![2, 7, 11, 15], 9)
                || vec![7, 2] == two_num_sum(&vec![2, 7, 11, 15], 9)
        );

        // Test with negative numbers
        assert!(
            vec![-1, 5] == two_num_sum(&vec![1, -1, 2, 5], 4)
                || vec![5, -1] == two_num_sum(&vec![1, -1, 2, 5], 4)
        );

        // Test with no solution
        assert_eq!(two_num_sum(&vec![1, 2, 3], 10), Vec::<i64>::new());

        // Test with duplicate numbers
        assert!(vec![2, 2] == two_num_sum(&vec![2, 2, 3], 4));
    }
}
