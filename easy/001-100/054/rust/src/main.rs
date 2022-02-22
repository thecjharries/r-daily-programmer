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

fn main() {
    println!("rad");
}

fn encode_matrix_cipher(message: String, columns: usize) -> String {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_matrix_cipher() {
        assert_eq!(
            encode_matrix_cipher("rad".to_string(), 2),
            "rda".to_string()
        );
        assert_eq!(
            encode_matrix_cipher("The cake is a lie!".to_string(), 3),
            "T kiaihces eea  l!".to_string()
        );
        assert_eq!(
            encode_matrix_cipher("The cake is a lie!".to_string(), 7),
            "Telh ieie s!c vaamk z".to_string()
        );
    }
}
