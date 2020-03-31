impl MorseDecoder {
    pub fn decode_bits(&self, encoded: &str) -> String {
        let deduped = encoded
            .chars()
            .enumerate()
            .filter(|(i, _)| i % 2 == 0)
            .map(|(_, c)| c);
        println!("{:?}", deduped);

        let mut frames = Vec::new();
        let mut frame = Vec::new();
        let mut prev_bit = None;

        loop {

        }

        unimplemented!()
    }

    fn decode_morse(&self, encoded: &str) -> String {
        let trimmed = encoded.trim();
        if trimmed == "" {
            String::new()
        } else {
            trimmed.split("   ").map(
                |word| word.split(" ").map(
                    |single_char| self.morse_code.get(single_char).unwrap()
                ).cloned().collect()
            ).fold(String::new(), |result, word| if result.len() == 0 {
                word
            } else {
                format!("{} {}", result, word)
            })
        }
    }
}

pub fn verify() {
    let decoder = MorseDecoder::new();
    assert_eq!(decoder.decode_bits("1100110011001100000011000000111111001100111111001111110000000000000011001111110011111100111111000000110011001111110000001111110011001100000011"), "···· · −·−−   ·−−− ··− −·· ");
}
