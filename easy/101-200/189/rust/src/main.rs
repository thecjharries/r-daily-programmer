// Copyright 2022 CJ Harries
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

struct HangMan {
    word: String,
    representation: String,
    guessed_letters: Vec<char>,
    guesses_remaining: usize,
}

impl HangMan {
    fn new(word: &str, guesses_remaining: usize) -> HangMan {
        HangMan {
            word: word.to_string(),
            representation: str::repeat("_", word.len()).to_string(),
            guessed_letters: Vec::new(),
            guesses_remaining,
        }
    }

    fn guess(&mut self, letter: char) -> &mut Self {
        if self.guessed_letters.contains(&letter) || 0 == self.guesses_remaining {
            return self;
        }
        self.guessed_letters.push(letter);
        for (i, c) in self.word.chars().enumerate() {
            if c == letter {
                self.representation.replace_range(i..i + 1, &c.to_string());
            }
        }
        self.guesses_remaining -= 1;
        self
    }
}

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hangman_new() {
        let game = HangMan::new("rad", 5);
        assert_eq!("rad", game.word);
        assert_eq!("___", game.representation);
        assert_eq!(5, game.guesses_remaining);
        assert_eq!(Vec::new() as Vec<char>, game.guessed_letters);
    }

    #[test]
    fn test_hangman_guess_correct() {
        let mut game = HangMan::new("rad", 5);
        game.guess('a');
        assert_eq!("rad", game.word);
        assert_eq!("_a_", game.representation);
        assert_eq!(4, game.guesses_remaining);
        assert_eq!(vec!['a'], game.guessed_letters);
    }

    #[test]
    fn test_hangman_guess_incorrect() {
        let mut game = HangMan::new("rad", 5);
        game.guess('b');
        assert_eq!("rad", game.word);
        assert_eq!("___", game.representation);
        assert_eq!(4, game.guesses_remaining);
        assert_eq!(vec!['b'], game.guessed_letters);
    }

    #[test]
    fn test_hangman_guess_none_remaining() {
        let mut game = HangMan::new("rad", 0);
        game.guess('a');
        assert_eq!("rad", game.word);
        assert_eq!("___", game.representation);
        assert_eq!(0, game.guesses_remaining);
        assert_eq!(Vec::new() as Vec<char>, game.guessed_letters);
    }

    #[test]
    fn test_hangman_guess_already_guessed() {
        let mut game = HangMan::new("rad", 5);
        game.guess('a');
        game.guess('a');
        assert_eq!("rad", game.word);
        assert_eq!("_a_", game.representation);
        assert_eq!(4, game.guesses_remaining);
        assert_eq!(vec!['a'], game.guessed_letters);
    }
}
