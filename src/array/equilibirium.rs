pub fn equilibirum(arr: Vec<i64>) -> i64 {
    if arr.is_empty() {
        return -1;
    }
    let total_sum: i64 = arr.iter().sum();
    let mut left_sum = 0;

    for (index, item) in arr.iter().enumerate() {
        let right_sum = total_sum - left_sum - item;
        if right_sum == left_sum {
            return index as i64;
        }
        left_sum += item;
    }
    return -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equilibirium() {
        assert_eq!(equilibirum(vec![1,2,0,3]), 2);
        assert_eq!(equilibirum(vec![1,1,1,1,1,1]), -1);
        assert_eq!(equilibirum(vec![1,1,1,1,1]), 2);
        assert_eq!(equilibirum(vec![-7, 1, 5, 2, -4, 3, 0]), 3);
    }
}