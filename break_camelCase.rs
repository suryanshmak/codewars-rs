fn solution(s: &str) -> String {
    let mut r = String::new();
    for ch in s.chars() {
        if ch.is_ascii_uppercase() {
          r.push(' ');
        }
      r.push(ch);
    };
    r
}
