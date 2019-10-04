// https://www.codewars.com/kata/5656b6906de340bd1b0000ac/train/rust

fn longest(a1: &str, a2: &str) -> String {
    let mut chars: Vec<_> = a1.chars().chain(a2.chars()).collect();

    chars.sort();
    chars.dedup();

    chars.into_iter().collect()
}

fn testing(s1: &str, s2: &str, exp: &str) -> () {
    println!("s1:{:?} s2:{:?}", s1, s2);
    println!("{:?} {:?}", longest(s1, s2), exp);
    println!("{}", longest(s1, s2) == exp);
    assert_eq!(&longest(s1, s2), exp)
}

pub fn verify() {
    testing("aretheyhere", "yestheyarehere", "aehrsty");
    testing("loopingisfunbutdangerous", "lessdangerousthancoding", "abcdefghilnoprstu");
}