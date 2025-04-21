use std::collections::HashMap;

fn balanced_paranthesis(data: &str) -> bool {
    // we will split the string
    let splitted_string:Vec<char> = data.chars().collect();
    let mut matching_table: HashMap<char, char> = HashMap::with_capacity(3);
    matching_table.insert('{', '}');
    matching_table.insert('(', ')');
    matching_table.insert('[', ']');

    let mut stack = Vec::new();
    // We need to mark if it's one of the open bracket then push into the stack
    // otherwise pop and match if it's matching the paranthesis or not
    for &i in splitted_string.iter() {
        match matching_table.contains_key(&i) {
            true => {stack.push(i);},
            false => {
                if let Some(popped_char) = stack.pop() {
                    if let Some(&matching_char) = matching_table.get(&popped_char) {
                        if matching_char != i {
                            return false
                        }
                    }
                } else {
                    return false
                }
            }
        }
    }

    if let Some(_) = stack.pop() {
        return false
    } else {
        true
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equilibirium() {
        assert_eq!(balanced_paranthesis("[{()}]"), true);
        assert_eq!(balanced_paranthesis("[()()]{}"), true);
        assert_eq!(balanced_paranthesis("([]"), false);
        assert_eq!(balanced_paranthesis("([{]})"), false);
    }
}
