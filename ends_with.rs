fn solution(w: &str, end: &str) -> bool {
  if w.len() < end.len() {
    false
  }
  
  &w[w.len()-end.len()..] == end
}

// w.ends_with(end) lmao
