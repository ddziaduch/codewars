fn dig_pow(n: i64, p: i32) -> i64 {
    let mut sum = 0;
    for i in n.to_string().chars() {
        let digit = i as i32;
        println!("{:?}", digit);
    }

    -1
}

//fn do_test(n: i64, p: i32, exp: i64) -> () {
//    println!(" n: {:?};", n);
//    println!("p: {:?};", p);
//    let ans = dig_pow(n, p);
//    println!(" actual:\n{:?};", ans);
//    println!("expect:\n{:?};", exp);
//    println!(" {};", ans == exp);
//    assert_eq!(ans, exp);
//    println!("{};", "-");
//}

pub fn verify() {
    dig_pow(89, 1);
//    do_test(89, 1, 1);
//    do_test(92, 1, -1);
//    do_test(46288, 3, 51);
}