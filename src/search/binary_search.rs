pub fn binary_search<T: Ord>(arr: &Vec<T>, target: &T) -> i64 {
    let mut left = 0;
    let mut right = arr.len() - 1;

    while right >= left {
        let mid = left + (right - left) / 2;
        if *target == arr[mid] {
            return mid as i64;
        }
        if arr[mid] > *target {
            right = mid - 1
        } else {
            left = mid + 1
        }
    }

    return -1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let arr = &vec![1, 2, 3, 4, 5, 6];
        let result = binary_search(arr, &6);
        assert_eq!(result, 5);
    }

    #[test]
    fn invalid_search_value() {
        let arr = &vec![1, 2, 3, 4, 5, 6];
        let result = binary_search(arr, &66);
        assert_eq!(result, -1)
    }
}
