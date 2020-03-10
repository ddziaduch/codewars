fn potatoes (initial_percent_of_water: i64, initial_weight_in_kg: i64, final_percent_of_water: i64) -> i64 {
    println!("{:?}, {:?}, {:?}", initial_percent_of_water, initial_weight_in_kg, final_percent_of_water);
    let initial_share_of_water: f64 = (100.0 - initial_percent_of_water as f64) / 100.0;
    let dry_mass_in_kg: f64 = initial_weight_in_kg as f64 * initial_share_of_water;
    let final_share_of_water: f64 = (100.0 - final_percent_of_water as f64) / 100.0;

    (dry_mass_in_kg / final_share_of_water + 0.00000000000001) as i64
}

fn dotest(p0: i64, w0: i64, p1: i64, exp: i64) -> () {
    let ans = potatoes(p0, w0, p1);
    assert_eq!(ans, exp)
}

pub fn verify() {
    dotest(99, 100, 98, 50);
    dotest(82, 127, 80, 114);
    dotest(94, 180, 90, 108);
}
