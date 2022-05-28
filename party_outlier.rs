fn outlier(values: &[i32]) -> i32 {
    let sum = values.iter().take(3).map(|x| x % 2).sum();

    let a = match sum {
        0 | 1 => 1,
        _ => 0
    };

    let val = values
        .iter()
        .find(|x| x.abs() % 2 == a);

    *val.unwrap()
}

fn main() {
    print!("{}", outlier(&[1, 3, 5, 7, 4]));
}
