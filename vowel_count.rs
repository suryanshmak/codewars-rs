fn get_count(string: &str) -> usize {
 string.matches(&['a', 'e', 'i', 'o', 'u'][..]).count() 
}
