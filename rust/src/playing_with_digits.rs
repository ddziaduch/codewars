use std::convert::TryInto;

fn dig_pow(n: i64, p: i32) -> i64 {
    let mut sum: i64 = 0;
    let mut exp = p.try_into().unwrap();
    let n_string: String = n.to_string();
    let digits = n_string.chars().map(|c| c.to_digit(10).unwrap() as i64);
    for digit in digits {
        println!("{:?}", digit);
        sum = sum + digit.pow(exp) as i64;
        exp = exp + 1;
    }
    let mut k: i64 = 1;

    while k * n <= sum {
        if k * n == sum {
            return k;
        } else {
            k = k + 1;
        }
    }

    -1
}

fn do_test(n: i64, p: i32, exp: i64) -> () {
    println!(" n: {:?};", n);
    println!("p: {:?};", p);
    let ans = dig_pow(n, p);
    println!(" actual:\n{:?};", ans);
    println!("expect:\n{:?};", exp);
    println!(" {};", ans == exp);
    assert_eq!(ans, exp);
    println!("{};", "-");
}

pub fn verify() {
    do_test(89, 1, 1);
    do_test(92, 1, -1);
    do_test(46288, 3, 51);
}