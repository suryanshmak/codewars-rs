fn xo(s: &str) -> bool {
    s.to_lowercase().matches("x").count() == s.to_lowercase().matches("o").count()
}
