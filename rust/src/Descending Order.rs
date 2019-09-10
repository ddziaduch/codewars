// https://www.codewars.com/kata/descending-order/train/rust

fn descending_order(x: u64) -> u64 {
    let mut y = x.to_string().chars().collect::<Vec<char>>();
    y.sort_by(|a, b| b.cmp(a));
    y.into_iter().collect::<String>().parse::<u64>().unwrap()
}

assert_eq!(descending_order(0), 0);
assert_eq!(descending_order(1), 1);
assert_eq!(descending_order(15), 51);
assert_eq!(descending_order(1021), 2110);
assert_eq!(descending_order(123456789), 987654321);
assert_eq!(descending_order(145263), 654321);
assert_eq!(descending_order(1254859723), 9875543221);