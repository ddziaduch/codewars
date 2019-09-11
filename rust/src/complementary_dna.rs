// https://www.codewars.com/kata/complementary-dna/train/rust

fn dna_strand(dna: &str) -> String {
    dna.chars().map(|nucleotide| match nucleotide {
        'A' => 'T',
        'T' => 'A',
        'C' => 'G',
        'G' => 'C',
        _ => panic!("DNA is invalid!")
    }).collect()
}

pub fn verify() {
    assert_eq!(dna_strand("AAAA"), "TTTT");
    assert_eq!(dna_strand("ATTGC"), "TAACG");
    assert_eq!(dna_strand("GTAT"), "CATA");
}