// https://www.codewars.com/kata/534d0a229345375d520006a0/train/rust

fn power_of_two(wanted: u64) -> bool {
    if wanted == 0 {
        false
    } else {
        let mut exponent: u32 = 0;
        loop {
            let current: u64 = 2_u64.pow(exponent);
            if current == wanted {
                break true;
            } else if current > wanted {
                break false;
            } else {
                exponent += 1;
            }
        }
    }
}

pub fn verify() {
    assert_eq!(power_of_two(0), false);
    assert_eq!(power_of_two(2), true);
    assert_eq!(power_of_two(5), false);
    assert_eq!(power_of_two(6), false);
    assert_eq!(power_of_two(8), true);
    assert_eq!(power_of_two(1024), true);
    assert_eq!(power_of_two(4096), true);
    assert_eq!(power_of_two(8191), false);
}
