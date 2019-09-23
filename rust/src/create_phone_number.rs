// https://www.codewars.com/kata/525f50e3b73515a6db000b83/train/rust

fn create_phone_number(n: &[u8]) -> String {
    format!(
        "({}{}{}) {}{}{}-{}{}{}{}",
        n[0],
        n[1],
        n[2],
        n[3],
        n[4],
        n[5],
        n[6],
        n[7],
        n[8],
        n[9]
    )
}

pub fn verify() {
    assert_eq!(create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 0]), "(123) 456-7890");
    assert_eq!(create_phone_number(&[1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), "(111) 111-1111");
    assert_eq!(create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 9]), "(123) 456-7899");
}