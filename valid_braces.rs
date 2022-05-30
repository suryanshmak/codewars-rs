fn valid_braces(s: &str) -> bool {
    let mut stack = vec![];
    for c in s.chars() {
        match c {
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            _ => {
               match stack.pop() {
                   Some(ch) => if ch != c {return false},
                   None => return false,
                }
            },
        }
    }
    stack.is_empty()
}
