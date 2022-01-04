fn main() {
    println!("Hello, world!");
}

fn encrypt_caesar(input: &str, shift: u32) -> String {
    let mut output = String::new();
    for input_character in input.chars() {
        let mut character = input_character;
        if character.is_alphabetic() {
            let mut character_code = character as u8;
            if character.is_lowercase() {
                character_code = character_code + shift as u8;
                if character_code > 'z' as u8 {
                    character_code = character_code - 26;
                }
            } else {
                character_code = character_code + shift as u8;
                if character_code > 'Z' as u8 {
                    character_code = character_code - 26;
                }
            }
            character = character_code as char;
        }
        output.push(character);
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_caesar() {
        assert_eq!(encrypt_caesar("A", 1), "B");
        assert_eq!(encrypt_caesar("A", 2), "C");
        assert_eq!(encrypt_caesar("A", 3), "D");
        assert_eq!(encrypt_caesar("A", 4), "E");
        assert_eq!(encrypt_caesar("A", 5), "F");
        assert_eq!(encrypt_caesar("A", 6), "G");
        assert_eq!(encrypt_caesar("A", 7), "H");
        assert_eq!(encrypt_caesar("A", 8), "I");
        assert_eq!(encrypt_caesar("A", 9), "J");
        assert_eq!(encrypt_caesar("A", 10), "K");
    }
}
