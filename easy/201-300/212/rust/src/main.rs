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

const CONSONANTS: &str = "bcdfghjklmnpqrtsvwxz";

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn convert_to_robbers_language(input: &str) -> String {
    input
        .chars()
        .map(|c| {
            if CONSONANTS.contains(c.to_ascii_lowercase()) {
                format!("{}o{}", c, c.to_ascii_lowercase())
            } else {
                c.to_string()
            }
        })
        .collect::<Vec<String>>()
        .join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(
            "Jojagog totalolaror Rorövovarorsospoproråkoketot!",
            convert_to_robbers_language("Jag talar Rövarspråket!")
        );
    }
}
