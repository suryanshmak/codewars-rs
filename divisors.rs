fn divisors(integer: u32) -> Result<Vec<u32>, String> {
        let mut arr: Vec<u32> = vec![];
        for i in 2..integer {
            if integer % i == 0 {
                arr.push(i);
            }
        }
        match arr.len() {
           0 => Err(format!("{} is prime", integer)),
           _ => Ok(arr)
        }
}
