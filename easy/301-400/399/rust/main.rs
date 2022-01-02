fn main() {
    assert_eq!(lettersum(""), 0);
    assert_eq!(lettersum("a"), 1);
    assert_eq!(lettersum("z"), 26);
    assert_eq!(lettersum("cab"), 6);
    assert_eq!(lettersum("excellent"), 100);
    assert_eq!(lettersum("microspectrophotometries"), 317);
}

fn lettersum(s: &str) -> u32 {
    s.chars().filter(|c| c.is_alphabetic()).map(|c| c.to_ascii_lowercase() as u32 - 'a' as u32 + 1).sum()
}
