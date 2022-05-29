fn get_middle(s:&str) -> &str {
    let l = s.len();
    &s[(l-1)/2..l/2+1]
}
