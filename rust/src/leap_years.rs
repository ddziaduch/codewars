// https://www.codewars.com/kata/leap-years/train/rust

fn is_leap_year(year: i32) -> bool {
    year % 400 == 0 || year % 4 == 0 && year % 100 != 0
}

pub fn verify() {
    assert_eq!(is_leap_year(1234), false);
    assert_eq!(is_leap_year(1984), true);
    assert_eq!(is_leap_year(2000), true);
    assert_eq!(is_leap_year(2010), false);
    assert_eq!(is_leap_year(2013), false);
}