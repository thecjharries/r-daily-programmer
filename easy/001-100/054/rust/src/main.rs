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

fn main() {
    println!("rad");
}

fn encode_matrix_cipher(message: String, columns: usize, rng: &mut Pcg64) -> String {}

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
