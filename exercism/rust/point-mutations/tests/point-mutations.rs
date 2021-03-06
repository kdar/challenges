extern crate point_mutations as dna;

#[test]
fn test_no_difference_between_empty_strands() {
    assert_eq!(dna::hamming_distance("", "").unwrap(), 0);
}

#[test]
fn test_no_difference_between_identical_strands() {
    assert_eq!(dna::hamming_distance("GGACTGA", "GGACTGA").unwrap(), 0);
}

#[test]
fn test_complete_hamming_distance_in_small_strand() {
    assert_eq!(dna::hamming_distance("ACT", "GGA").unwrap(), 3);
}

#[test]
fn test_small_hamming_distance_in_the_middle_somewhere() {
    assert_eq!(dna::hamming_distance("GGACG", "GGTCG").unwrap(), 1);
}

#[test]
fn test_larger_distance() {
    assert_eq!(dna::hamming_distance("ACCAGGG", "ACTATGG").unwrap(), 2);
}

#[test]
fn test_first_string_is_longer() {
    assert_eq!(dna::hamming_distance("AAA", "AA"), Result::Err("inputs of different length"));
}

#[test]
fn test_second_string_is_longer() {
    assert_eq!(dna::hamming_distance("A", "AA"), Result::Err("inputs of different length"));
}
