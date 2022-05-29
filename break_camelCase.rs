fn solution(s: &str) -> String {
    let mut result = String::new();
    for ch in s.chars() {
        if ch.is_ascii_uppercase() {
          result.push(' ');
        }
      result.push(ch);
    };
    result
}
