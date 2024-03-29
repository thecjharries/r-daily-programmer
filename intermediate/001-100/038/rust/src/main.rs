// Copyright 2023 CJ Harries
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

const OPERATORS: [char; 5] = ['+', '-', '*', '/', '^'];

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn reverse_polish_notation(input: &str) -> String {
    let mut operators: Vec<char> = Vec::new();
    let mut chunks: Vec<String> = Vec::new();
    let input = input.replace(" ", "");
    for character in input.chars() {
        if OPERATORS.contains(&character) {
            operators.push(character);
        } else if ')' == character {
            let chunk = chunks.pop().unwrap();
            let operator = operators.pop().unwrap();
            let previous_chunk = chunks.pop().unwrap();
            chunks.push(format!("{}{}{}", previous_chunk, chunk, operator));
        } else if '(' != character {
            chunks.push(character.to_string());
        }
    }
    chunks[0].clone()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_polish_notation() {
        assert_eq!("abc*+".to_string(), reverse_polish_notation("(a+(b*c))"));
        assert_eq!(
            "ab+zx+*".to_string(),
            reverse_polish_notation("((a+b)*(z+x))")
        );
        assert_eq!(
            "at+bac++cd+^*".to_string(),
            reverse_polish_notation("((a+t)*((b+(a+c)) ^ (c+d)))")
        );
    }
}
