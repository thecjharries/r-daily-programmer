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

fn sorta_sort_prompt(input: Vec<String>) -> Vec<String> {
    let mut output = Vec::new();
    let mut remaining = Vec::new();
    for item in input {
        if item.starts_with("#") {
            output.push(item);
        } else {
            remaining.push(item);
        }
    }
    let mut temporary = Vec::new();
    let mut quads = Vec::new();
    for item in remaining {
        if item.starts_with("    ") {
            quads.push(item);
        } else {
            temporary.push(item);
        }
    }
    remaining = temporary;
    temporary = Vec::new();
    let mut dubs = Vec::new();
    for item in remaining {
        if item.starts_with("  ") {
            dubs.push(item);
        } else {
            temporary.push(item);
        }
    }
    output.append(&mut temporary);
    output.append(&mut dubs);
    output.append(&mut quads);
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(
            vec![
                "#include <iostream>".to_string(),
                "{".to_string(),
                "}".to_string(),
                "int main()".to_string(),
                "  {".to_string(),
                "  }".to_string(),
                "  int sum = 0".to_string(),
                "  for (int i = 0; i <= 100; ++i)".to_string(),
                "  std::cout << sum;".to_string(),
                "  return 0;".to_string(),
                "    sum = i + sum;".to_string(),
            ],
            sorta_sort_prompt(vec![
                "    sum = i + sum;".to_string(),
                "  {".to_string(),
                "  }".to_string(),
                "  int sum = 0".to_string(),
                "  for (int i = 0; i <= 100; ++i)".to_string(),
                "  std::cout << sum;".to_string(),
                "  return 0;".to_string(),
                "{".to_string(),
                "}".to_string(),
                "#include <iostream>".to_string(),
                "int main()".to_string(),
            ])
        );
    }
}
