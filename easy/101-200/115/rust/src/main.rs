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

use rand::prelude::*;
use rand_pcg::Pcg64;
use std::io::{BufRead, Read, Write};

fn main() {
    println!("rad");
}

fn play_guessing_game<R: Rng>(
    rng: &mut R,
    input: &mut (impl Read + BufRead),
    output: &mut impl Write,
) {
    let mut guess = String::new();
    let secret = rng.gen_range(1..101);
    loop {
        output
            .write(b"Guess a number between 1 and 100: \n")
            .unwrap();
        output.flush().unwrap();
        input.read_line(&mut guess).unwrap();
        if guess.ends_with("exit\n") {
            break;
        }
        let guess = guess.trim().parse::<u32>().unwrap();
        if guess == secret {
            output.write(b"You win!\n").unwrap();
            break;
        } else if guess < secret {
            output.write(b"Too low!\n").unwrap();
        } else {
            output.write(b"Too high!\n").unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_play_guessing_game() {
        let mut rng = Pcg64::seed_from_u64(0);
        let mut output: Vec<u8> = Vec::new();
        play_guessing_game(&mut rng, &mut "50\nexit\n".as_bytes(), &mut output);
        assert_eq!(
            output,
            b"Guess a number between 1 and 100: \nToo low!\nGuess a number between 1 and 100: \n"
        );
        output.clear();
        play_guessing_game(&mut rng, &mut "50\nexit\n".as_bytes(), &mut output);
        assert_eq!(
            output,
            b"Guess a number between 1 and 100: \nToo high!\nGuess a number between 1 and 100: \n"
        );
        output.clear();
        play_guessing_game(&mut rng, &mut "50\nexit\n".as_bytes(), &mut output);
        assert_eq!(
            output,
            b"Guess a number between 1 and 100: \nToo high!\nGuess a number between 1 and 100: \n"
        );
        output.clear();
        play_guessing_game(&mut rng, &mut "exit\n".as_bytes(), &mut output);
        assert_eq!(output, b"Guess a number between 1 and 100: \n");
    }
}
