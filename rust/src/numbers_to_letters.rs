fn switcher(numbers: Vec<&str>) -> String {
    numbers.into_iter().map(|numeric| {
        match numeric {
            "27" => '!',
            "28" => '?',
            "29" => ' ',
            _ => (27 - match numeric.parse::<u8>() {
                Ok(x) => x,
                _ => panic!("Unexpected number {:?}", numeric)
            } + 96) as char
        }
    }).collect()
}

pub fn verify() {
    assert_eq!(switcher(vec!["27", "28", "29"]), "!? ");
    assert_eq!(switcher(vec!["4", "24"]), "wc");
    assert_eq!(switcher(vec!["24", "12", "23", "22", "4", "26", "9", "8"]), "codewars");
    assert_eq!(switcher(vec!["25","7","8","4","14","23","8","25","23","29","16","16","4"]), "btswmdsbd kkw");
}
