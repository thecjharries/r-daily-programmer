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

fn generate_abacaba_sequence(iteration: u8) -> String {
    let mut sequence = String::from("a");
    for index in 1_u8..iteration {
        let ending = ((index % 26 + 'a' as u8) as char).to_string() + &sequence;
        sequence = sequence + &ending;
    }
    sequence
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_abacaba_sequence() {
        assert_eq!(generate_abacaba_sequence(1), "a".to_string());
        assert_eq!(generate_abacaba_sequence(2), "aba".to_string());
        assert_eq!(generate_abacaba_sequence(3), "abacaba".to_string());
        assert_eq!(generate_abacaba_sequence(4), "abacabadabacaba".to_string());
    }
}
