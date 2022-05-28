fn divisors(integer: u32) -> Result<Vec<u32>, String> {
        let mut arr: Vec<u32> = (2..integer).filter(|x| integer % x == 0).collect();
        match arr.len() {
           0 => Err(format!("{} is prime", integer)),
           _ => Ok(arr)
        }
}
