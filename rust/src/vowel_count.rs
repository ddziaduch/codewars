const VOWELS: [char; 5] = ['a','e','i','o', 'u'];

fn get_count(string: &str) -> usize {
    string.chars().filter(|x: &char| VOWELS.contains(x)).count()
}

pub fn verify() {
    assert_eq!(get_count("abracadabra"), 5);
}
