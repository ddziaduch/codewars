fn count_bits(n: i64) -> u32 {
    format!("{:b}", n)
        .chars()
        .into_iter()
        .filter(|char| char == &'1')
        .collect::<Vec<_>>()
        .len() as u32
}

pub fn verify() {
    assert_eq!(count_bits(0), 0);
    assert_eq!(count_bits(4), 1);
    assert_eq!(count_bits(7), 3);
    assert_eq!(count_bits(9), 2);
    assert_eq!(count_bits(10), 2);
}