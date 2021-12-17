// https://www.codewars.com/kata/515de9ae9dcfc28eb6000001/train/rust

fn solution(s: &str) -> Vec<String> {
    let mut v: Vec<String> = Vec::new();
    let mut iter = s.chars();
    loop {
        let next = iter.next();
        let next_next = iter.next();
        if next.is_none() {
            break;
        }

        let mut s = String::new();

        s.push(next.unwrap_or('_'));
        s.push(next_next.unwrap_or('_'));

        v.push(s);
    }
    v
}

pub fn verify() {
    assert_eq!(solution("abcdef"), ["ab", "cd", "ef"]);
    assert_eq!(solution("abcdefg"), ["ab", "cd", "ef", "g_"]);
    assert_eq!(solution(""), [] as [&str; 0]);
}
