pub fn remove_char(s: &str) -> String {
    let mut res: Vec<char> = Vec::new();

    for (i, c) in s.chars().enumerate() {
        if i != 0 && i != (s.len() - 1) {
            res.push(c);
        }
    }

    res.iter().collect()
}

pub fn verify() {
    assert_eq!(remove_char("eloquent"), "loquen");
    assert_eq!(remove_char("country"), "ountr");
    assert_eq!(remove_char("person"), "erso");
    assert_eq!(remove_char("place"), "lac");
    assert_eq!(remove_char("ok"), "");
    assert_eq!(remove_char("ooopsss"), "oopss");
}
