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

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn build_house(input: &str) -> String {
    let rows = input.split("\n").collect::<Vec<&str>>();
    let mut result: Vec<String> = vec!["".to_string(); rows.len() * 2];
    for (index, row) in rows.iter().enumerate() {
        for c in row.chars() {
            match c {
                '*' => {
                    result[index * 2].push_str("+---+");
                    result[index * 2 + 1].push_str("|   |");
                }
                _ => {
                    result[2 * index].push_str("     ");
                    result[2 * index + 1].push_str("     ");
                }
            }
        }
    }
    result.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!("               +---+\n               |   |\n          +---++---++---+\n          |   ||   ||   |\n+---++---++---++---++---++---+\n|   ||   ||   ||   ||   ||   |", build_house("   *\n  ***\n******"));
    }
}
