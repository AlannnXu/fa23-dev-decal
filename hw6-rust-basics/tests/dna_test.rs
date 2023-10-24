// Бележка: името на проекта трябва да се казва "solution". Ако не се казва така, променете го
// на този ред:
use hw6_rust_basics::solution;

#[test]
fn test_counts_zero() {
    let counter = solution::counts(&[]);

    assert_eq!(counter.g, 0);
    assert_eq!(counter.c, 0);
    assert_eq!(counter.a, 0);
    assert_eq!(counter.t, 0);
}

#[test]
fn test_counts_basic() {
    let counter = solution::counts(&['A', 'G', 'C', 'T']);

    assert_eq!(counter.g, 1);
    assert_eq!(counter.c, 1);
    assert_eq!(counter.a, 1);
    assert_eq!(counter.t, 1);
}

#[test]
#[should_panic]
fn test_counts_panic1() {
    solution::counts(&['A', 'X', 'C']);
}

#[test]
#[should_panic]
fn test_counts_panic2() {
    solution::counts(&['a', 'g', 'c']);
}

#[test]
fn test_counts_big() {
    let input: Vec<char> =
        "AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC".chars().collect();
    let counter = solution::counts(&input);

    assert_eq!(counter.g, 17);
    assert_eq!(counter.c, 12);
    assert_eq!(counter.a, 20);
    assert_eq!(counter.t, 21);
}

#[test]
fn test_dna_complement_empty() {
    assert_eq!(solution::dna_complement(&[]), Vec::new());
}

#[test]
#[should_panic]
fn test_dna_complement_panic() {
    solution::dna_complement(&['X']);
}

#[test]
fn test_dna_complement_big() {
    let input: Vec<char> =
        "AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC".chars().collect();
    let expected_output: Vec<char> =
        "TCGAAAAGTAAGACTGACGTTGCCCGTTATACAGAGACACACCTAATTTTTTTCTCACAGACTATCGTCG".chars().collect();
    assert_eq!(solution::dna_complement(&input), expected_output);
}

#[test]
fn test_reverse_rna_complement_empty() {
    assert_eq!(solution::reverse_rna_complement(&[]), Vec::new());
}

#[test]
#[should_panic]
fn test_reverse_rna_complement_panic1() {
    solution::reverse_rna_complement(&['X']);
}

#[test]
#[should_panic]
fn test_reverse_rna_complement_panic2() {
    solution::reverse_rna_complement(&['U']);
}

#[test]
fn test_reverse_rna_complement_big() {
    let input: Vec<char> =
        "AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC".chars().collect();
    let mut expected_output: Vec<char> =
        "UCGAAAAGUAAGACUGACGUUGCCCGUUAUACAGAGACACACCUAAUUUUUUUCUCACAGACUAUCGUCG".chars().collect();
    expected_output.reverse();

    assert_eq!(solution::reverse_rna_complement(&input), expected_output);
}