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

use rand::distributions::Alphanumeric;
use rand::prelude::*;
use rand_pcg::Pcg64;

fn main() {
    println!("rad");
}

fn encode_matrix_cipher(message: String, columns: usize, rng: &mut Pcg64) -> String {
    let desired_length = columns * ((message.len() as f64 / columns as f64).ceil() as usize);
    let padding = rng
        .sample_iter(&Alphanumeric)
        .take(desired_length - message.len())
        .map(char::from)
        .collect::<String>();
    let plaintext = message + &padding;
    let mut ciphertext = String::new();
    for i in 0..columns {
        for j in 0..plaintext.len() / columns {
            ciphertext.push(plaintext.chars().nth(i + j * columns).unwrap());
        }
    }
    ciphertext
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_matrix_cipher() {
        let mut rng = Pcg64::seed_from_u64(0);
        assert_eq!(
            encode_matrix_cipher("rad".to_string(), 2, &mut rng),
            "rda".to_string()
        );
        assert_eq!(
            encode_matrix_cipher("The cake is a lie!".to_string(), 3, &mut rng),
            "T kiaihces eea  l!".to_string()
        );
        assert_eq!(
            encode_matrix_cipher("The cake is a lie!".to_string(), 7, &mut rng),
            "Telh ieie s!c vaamk z".to_string()
        );
    }
}
