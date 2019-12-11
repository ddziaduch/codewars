fn row_sum_odd_numbers(n: i64) -> i64 {
    let mut sum = 0;
    let mut odd = n * n - (n - 1);
    let mut i = 0;

    while i < n {
        sum += odd;
        odd += 2;
        i += 1;
    }

    sum
}

pub fn verify() {
    assert_eq!(row_sum_odd_numbers(1), 1);
    assert_eq!(row_sum_odd_numbers(2), 8);
    assert_eq!(row_sum_odd_numbers(3), 7 + 9 + 11);
    assert_eq!(row_sum_odd_numbers(4), 13 + 15 + 17 + 19);
    assert_eq!(row_sum_odd_numbers(5), 21 + 23 + 25 + 27 + 29);
    assert_eq!(row_sum_odd_numbers(42), 74088);
}
