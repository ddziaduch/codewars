// https://www.codewars.com/kata/54b72c16cd7f5154e9000457/train/rust

struct MorseDecoder {
    // morse_code:
}

impl MorseDecoder {
    pub fn decode_bits(encoded: &str) -> String {
        let deduped = encoded
            .chars()
            .enumerate()
            .filter(|(i, _)| i % 2 == 0)
            .map(|(_, c)| c);
        println!("{:?}", deduped);

        let mut result = Vec::new();

        loop {
            let is_pause_between_words: bool = {
                deduped.take(7).fold(String::new(), |acc, c| acc + &String::from(c)) ==
                    String::from("0000000")
            };

            if is_pause_between_words {
                deduped.
            }

            let is_pause_between_characters_inside_a_word: bool = {
                for bit in deduped.take(3) {
                    if bit != '0' {
                        false
                    }
                }
                true
            };

            let is_pause_between_dots_and_dashes_in_a_character: bool = {
                for bit in deduped.take(1) {
                    if bit != '0' {
                        false
                    }
                }
                true
            };

            let is_pause_between_dots_and_dashes_in_a_character: bool = {
                for bit in deduped.take(1) {
                    if bit != '0' {
                        false
                    }
                }
                true
            };
        }

        unimplemented!()
    }

    // fn decode_morse(&self, encoded: &str) -> String {
    //     let trimmed = encoded.trim();
    //     if trimmed == "" {
    //         String::new()
    //     } else {
    //         trimmed.split("   ").map(
    //             |word| word.split(" ").map(
    //                 |single_char| self.morse_code.get(single_char).unwrap()
    //             ).cloned().collect()
    //         ).fold(String::new(), |result, word| if result.len() == 0 {
    //             word
    //         } else {
    //             format!("{} {}", result, word)
    //         })
    //     }
    // }
}

pub fn verify() {
    // let decoder = MorseDecoder::new();
    assert_eq!(MorseDecoder::decode_bits
    ("1100110011001100000011000000111111001100111111001111110000000000000011001111110011111100111111000000110011001111110000001111110011001100000011"), "···· · −·−−   ·−−− ··− −·· ");
}
