fn main() {
    println!("Hello, world!");
}

fn lettersum(s: &str) -> u32 {
    s.chars().filter(|c| c.is_alphabetic()).map(|c| c.to_ascii_lowercase() as u32 - 'a' as u32 + 1).sum()
}
