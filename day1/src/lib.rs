pub fn calculate_frequency(data: &str) -> i32 {
    let mut num = 0i32;

    for line in data.lines() {
        match i32::from_str_radix(line.trim(), 10) {
            Ok(x) => num += x,
            _ => (),
        }
    }

    num
}
