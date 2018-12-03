use std::collections::HashSet;

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

pub fn find_twice_frequency(data: &str) -> Option<i32> {
    let mut num = 0i32;
    let mut values = HashSet::new();

    loop {
        for line in data.lines() {
            match i32::from_str_radix(line.trim(), 10) {
                Ok(x) => {
                    num += x;
                    if !values.contains(&num) {
                        values.insert(num);
                    } else {
                        return Some(num);
                    }
                }
                _ => (),
            }
        }
    }
}
