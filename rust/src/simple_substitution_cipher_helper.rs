// https://www.codewars.com/kata/simple-substitution-cipher-helper/rust

struct Cipher {
    input: String,
    output: String
}

impl Cipher {
    fn new(map1: &str, map2: &str) -> Cipher {
        Cipher {
            input: map1.to_string(),
            output: map2.to_string()
        }
    }

    fn encode(&self, string: &str) -> String {
        translate(&self.input, &self.output, string)
    }

    fn decode(&self, string: &str) -> String {
        translate(&self.output, &self.input, string)
    }
}

fn translate(input: &String, output: &String, string: &str) -> String {
    string.chars().map(|char| {
        match input.find(char) {
            Some(index) => output.chars().nth(index).unwrap(),
            None => char,
        }
    }).collect()
}

pub fn verify() {
    let map1 = "abcdefghijklmnopqrstuvwxyz";
    let map2 = "etaoinshrdlucmfwypvbgkjqxz";

    let cipher = Cipher::new(map1, map2);

    assert_eq!(cipher.encode("abc"), "eta");
    assert_eq!(cipher.encode("xyz"), "qxz");
    assert_eq!(cipher.decode("eirfg"), "aeiou");
    assert_eq!(cipher.decode("erlang"), "aikcfu");
}
