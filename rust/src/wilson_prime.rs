fn am_i_wilson(n: u32) -> bool {
    // I tried to implement this but then I realized that
    // there are only three numbers known on Wiki :)
    [5, 13, 563].contains(&n)
}

pub fn verify() {
    assert_eq!(am_i_wilson(9), false);
    assert_eq!(am_i_wilson(6), false);

    assert_eq!(am_i_wilson(2), false);
    assert_eq!(am_i_wilson(17), false);

    assert_eq!(am_i_wilson(5), true);

    assert_eq!(am_i_wilson(307), false);

    assert_eq!(am_i_wilson(5971), false);
}