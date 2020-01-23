fn solution(phrase: &str) -> String {
    phrase.chars().rev().collect()
}

pub fn verify() {
    assert_eq!(solution("world"), "dlrow");
}
