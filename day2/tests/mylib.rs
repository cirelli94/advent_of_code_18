extern crate mylib;

use mylib::*;

#[test]
fn test_zero() {
    assert_eq!(count_letters(""), (0, 0));
    assert_eq!(count_letters("abcdef"), (0, 0));
}

#[test]
fn test_one() {
    assert_eq!(count_letters("abbcde"), (1, 0));
    assert_eq!(count_letters("abcccd"), (0, 1));
    assert_eq!(count_letters("abcdee"), (1, 0));
}

#[test]
fn test_multiple() {
    assert_eq!(count_letters("aabcdd"), (1, 0));
    assert_eq!(count_letters("ababab"), (0, 1));
}

#[test]
fn test_both() {
    assert_eq!(count_letters("ababb"), (1, 1));
}
