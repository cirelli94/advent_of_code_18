extern crate mylib;

use mylib::*;

#[test]
fn test_empty() {
    assert_eq!(calculate_frequency(""), 0);
}

#[test]
fn test_zero() {
    assert_eq!(calculate_frequency("+0"), 0);
}

#[test]
fn test_one_positive_number() {
    assert_eq!(calculate_frequency("+3"), 3);
}

#[test]
fn test_one_negative_number() {
    assert_eq!(calculate_frequency("-42"), -42);
}
