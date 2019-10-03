// https://www.codewars.com/kata/century-from-year/rust

fn century(year: u32) -> u32 {
    if year % 100 == 0 {
        year / 100
    } else {
        year / 100 + 1
    }
}

pub fn verify() {
    assert_eq!(century(1705), 18);
    assert_eq!(century(1900), 19);
    assert_eq!(century(1601), 17);
    assert_eq!(century(2000), 20);
    assert_eq!(century(89), 1);
    assert_eq!(century(101), 2);
}