fn solution(n: i32) -> i32 {
    (1..n).filter(|n| n % 3 == 0 || n % 5 == 0).fold(0, |acc, n| acc + n)
}

pub fn verify() {
    assert_eq!(solution(10), 23);
    assert_eq!(solution(11), 33);
    assert_eq!(solution(6), 8);
}
