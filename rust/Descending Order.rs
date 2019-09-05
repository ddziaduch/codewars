// https://www.codewars.com/kata/descending-order/train/rust

fn descending_order(number: u64) -> u64 {
    let string = number.to_string();
    let chars = string.chars();
    let digits = chars.map(|digit| digit.to_digit(10).unwrap());
    let mut vector = digits.collect::<Vec<u32>>();
    vector.sort_by(|a, b| b.partial_cmp(a).unwrap());
    vector.into_iter().map(|y| y.to_string()).collect::<String>().parse::<u64>().ok().unwrap()
}

assert_eq!(descending_order(0), 0);
assert_eq!(descending_order(1), 1);
assert_eq!(descending_order(15), 51);
assert_eq!(descending_order(1021), 2110);
assert_eq!(descending_order(123456789), 987654321);
assert_eq!(descending_order(145263), 654321);
assert_eq!(descending_order(1254859723), 9875543221);