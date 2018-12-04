use std::collections::BTreeMap;

pub fn count_letters(str: &str) -> (usize, usize) {
    let mut map = BTreeMap::new();

    for c in str.chars() {
        if !map.contains_key(&c) {
            map.insert(c, 1);
            continue;
        }

        if let Some(x) = map.get_mut(&c) {
            *x = *x + 1;
        }
    }

    let mut x = 0;
    let mut y = 0;

    if map.iter().any(|(_, t)| *t == 2) {
        x = 1;
    }
    if map.iter().any(|(_, t)| *t == 3) {
        y = 1;
    }

    (x, y)
}

// Return true if only on charater is different between the two strings
pub fn ids_prototypes(str1: &str, str2: &str) -> bool {
    // if str1.len() != str2.len() {
    //     return false;
    // }

    // let mut chars2 = str2.chars();

    // let mut errors = 0;

    // for c in str1.chars() {
    //     if c != chars2.next().unwrap() {
    //         errors += 1;

    //         if errors > 1 {
    //             return false;
    //         }
    //     }
    // }

    // true
    str1.chars()
        .zip(str2.chars())
        .filter(|(a, b)| a != b)
        .count()
        == 1
}
