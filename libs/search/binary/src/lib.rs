pub fn binary_search(arr: &Vec<i32>, data: &i32) -> i32 {
    return binary(arr, 0, arr.len() - 1, data);
}

pub fn binary(arr: &Vec<i32>, start_index: usize, end_index: usize, data: &i32) -> i32 {
    if arr[start_index] == *data {
        return start_index as i32;
    }
    if start_index == end_index {
        return -1;
    }
    let mid = (start_index + end_index)/2;

    if data == &arr[mid] {
        return mid as i32;
    }
    if data > &arr[mid] {
        return binary(arr, mid + 1, end_index, data);
    }

    if data < &arr[mid] {
        return binary(arr, start_index, mid -1, data);
    }
    return -1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let arr = &vec![1,2,3,4,5,6];
        let result = binary_search(arr, &7);
        assert_eq!(result, -1);
    }
}
