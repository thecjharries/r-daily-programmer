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

fn is_valid_symbol(element: &str, symbol: &str) -> bool {
    let symbol_chars: Vec<char> = symbol.to_lowercase().chars().collect();
    if 2 != symbol_chars.len() {
        return false;
    }
    let first_index = element
        .to_lowercase()
        .find(symbol_chars[0])
        .unwrap_or(usize::MAX);
    let last_index = element
        .to_lowercase()
        .rfind(symbol_chars[1])
        .unwrap_or(usize::MIN);
    first_index < last_index
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_symbol() {
        assert_eq!(true, is_valid_symbol("Spenglerium", "Ee"));
        assert_eq!(true, is_valid_symbol("Zeddemorium", "Zr"));
        assert_eq!(true, is_valid_symbol("Venkmine", "Kn"));
        assert_eq!(false, is_valid_symbol("Stantzon", "Zt"));
        assert_eq!(false, is_valid_symbol("Melintzum", "Nn"));
        assert_eq!(false, is_valid_symbol("Tullium", "Ty"));
    }
}
