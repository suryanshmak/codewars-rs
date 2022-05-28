fn outlier(values: &[i32]) -> i32 {
  let sum = values.iter().take(3).map(|x| x%2).sum();
  
  let a = match sum {
    0 | 1 => 1,
    _ => 0
  }
  
  values
      .iter()
      .find(|x| x.abs() % 2 == a)
      .map(|x| *x)
      .unwrap_or(0)
}
