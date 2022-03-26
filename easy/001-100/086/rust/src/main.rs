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

fn compress_run_length(input: &str) -> Vec<(i32, char)> {
    let mut result = Vec::new();
    let mut count = 0;
    let mut last_char = ' ';
    for c in input.chars() {
        if c == last_char {
            count += 1;
        } else {
            if count > 0 {
                result.push((count, last_char));
            }
            count = 1;
            last_char = c;
        }
    }
    if count > 0 {
        result.push((count, last_char));
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compress_run_length() {
        assert_eq!(
            compress_run_length("Heeeeelllllooooo nurse!"),
            vec![
                (1, 'H'),
                (5, 'e'),
                (5, 'l'),
                (5, 'o'),
                (1, ' '),
                (1, 'n'),
                (1, 'u'),
                (1, 'r'),
                (1, 's'),
                (1, 'e'),
                (1, '!')
            ]
        )
    }
}
