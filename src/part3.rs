use std::collections::BTreeMap;

mod extended_lexer;

fn aho_corasick_failure(input: &str) -> String{
    let input_as_chars: Vec<char> = input.chars().collect();
    let mut result = BTreeMap::new();
    let mut t = 0;
    result.insert(1, 0);

    for s in 1..input.len() {
        while t > 0 && input_as_chars[s] != input_as_chars[t] {
            t = *(result.get(&t).unwrap());
        }
        if input_as_chars[s] == input_as_chars[t] {
            t += 1;
            result.insert(s + 1, t);
        } else {
            result.insert(s + 1, 0);
        }
    };
    result.iter().fold("".to_string(),|accum, next|{
        accum+&next.1.to_string()
    })


}

#[cfg(test)]
mod tests {
    #[test]
    fn aho_corasick_failure_test() {
        use super::aho_corasick_failure;
        assert_eq!(aho_corasick_failure("ababaa"),"001231");
        assert_eq!(aho_corasick_failure("abababaab"),"001234512");
        assert_eq!(aho_corasick_failure("aaaaaa"),"012345");
        assert_eq!(aho_corasick_failure("abbaabb"),"0001123");
    }
}