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

fn find_possible_decodes(number_as_string: String, working_string: String) -> Vec<String> {
    let mut current_number_as_string = number_as_string.clone();
    if 0 == number_as_string.len() {
        return vec![working_string];
    }
    let mut current_code = String::from(current_number_as_string.remove(0));
    while "0".to_string() == current_code && number_as_string.len() > 0 {
        current_code = String::from(current_number_as_string.remove(0));
    }
    if "0".to_string() == current_code {
        return vec![working_string];
    }
    let mut current_working_string = working_string.clone();
    current_working_string.push(('a' as u8 + current_code.parse::<u8>().unwrap() - 1) as char);
    let mut decodes = find_possible_decodes(current_number_as_string, current_working_string);
    if 1 < number_as_string.len() {
        current_code = String::from(&number_as_string[0..2]);
        let converted = current_code.parse::<u8>().unwrap();
        if 27 > converted {
            let mut current_working_string = working_string.clone();
            current_working_string.push(('a' as u8 + converted - 1) as char);
            let mut decodes2 =
                find_possible_decodes(number_as_string[2..].to_string(), current_working_string);
            decodes.append(&mut decodes2);
        }
    }
    decodes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_possible_decodes() {
        assert_eq!(
            find_possible_decodes("123".to_string(), String::new()),
            vec!["abc".to_string(), "aw".to_string(), "lc".to_string()]
        );
        assert_eq!(
            find_possible_decodes("85121215".to_string(), String::new()),
            vec![
                "heababae".to_string(),
                "heababo".to_string(),
                "heabaue".to_string(),
                "heablae".to_string(),
                "heablo".to_string(),
                "heaubae".to_string(),
                "heaubo".to_string(),
                "heauue".to_string(),
                "helabae".to_string(),
                "helabo".to_string(),
                "helaue".to_string(),
                "hellae".to_string(),
                "hello".to_string()
            ]
        );
    }
}
