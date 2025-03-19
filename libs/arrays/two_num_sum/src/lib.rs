use std::collections::HashMap;

pub fn two_num_sum(arr: &Vec<i64>, target: i64) -> Vec<i64> {
    let mut target_arr: Vec<i64> = Vec::new();
    let mut num_map = HashMap::new();

    for (index, &data) in arr.iter().enumerate() {
        num_map.insert(data, index);
    }

    for (index, &data) in arr.iter().enumerate() {
        let complement = target - data;
        println!("{data}, {complement}");
        if let Some(&complement_index) = num_map.get(&complement) {
            if complement_index != index {
                target_arr.push(data);
                target_arr.push(complement);
                return target_arr;
            }
        }
    }

    return target_arr;
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
