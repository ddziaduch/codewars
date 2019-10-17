// https://www.codewars.com/kata/good-vs-evil/rust

use std::cmp::Ordering;

fn good_vs_evil(good: &str, evil: &str) -> String {
    let good_worth = [1, 2, 3, 3, 4, 10];
    let evil_worth = [1, 2, 2, 2, 3, 5, 10];

    let good_score = get_score(good, &good_worth);
    let evil_score = get_score(evil, &evil_worth);

    match good_score.cmp(&evil_score) {
        Ordering::Greater => "Battle Result: Good triumphs over Evil",
        Ordering::Equal => "Battle Result: No victor on this battle field",
        Ordering::Less => "Battle Result: Evil eradicates all trace of Good"
    }.to_string()
}

fn get_score(input: &str, worth_array: &[i32]) -> i32 {
    input.split(' ').map(|count: &str| -> i32 {
        count.parse().unwrap()
    }).enumerate().fold(0, |sum: i32, (index, count): (usize, i32)| -> i32 {
        sum + worth_array[index] * count
    })
}

pub fn verify() {
    assert_eq!(good_vs_evil("0 0 0 0 0 10", "0 0 0 0 0 0 0"), "Battle Result: Good triumphs over Evil");
    assert_eq!(good_vs_evil("0 0 0 0 0 0", "0 0 0 0 0 0 10"), "Battle Result: Evil eradicates all trace of Good");
    assert_eq!(good_vs_evil("0 0 0 0 0 10", "0 0 0 0 0 0 10"), "Battle Result: No victor on this battle field");
}