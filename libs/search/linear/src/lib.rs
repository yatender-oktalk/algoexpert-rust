pub fn linear_search<T: Ord>(arr: &Vec<T>, data_to_search: &T) -> i32 {
    for (idx, data) in arr.into_iter().enumerate() {
        if data == data_to_search {
            return idx as i32
        }
    }
    return -1;
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn search_num() {
        let arr = vec![1,2,3,4,5];
        let result = linear_search(&arr, &5);
        print!("{}", result);
        assert_eq!(result, 4);
    }

    #[test]
    fn search_string() {
        let arr = vec!["a", "b", "cd", "ef"];
        let result = linear_search(&arr, &"ef");
        assert_eq!(result, 3);
    }

    #[test]
    fn not_found_case_string() {
        let arr = vec!["a", "b", "cd", "ef"];
        let result = linear_search(&arr, &"efg");
        assert_eq!(result, -1);
    }
}
