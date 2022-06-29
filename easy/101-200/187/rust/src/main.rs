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

fn parse_flags(available_flags: Vec<&str>, input: &str) -> Vec<String> {
    let mut flags = Vec::new();
    for chunk in input.split_whitespace() {
        if chunk.starts_with("--") {
            let flag = &chunk[2..];
            flags.push(format!("flag: {}", flag.to_string()));
        } else if chunk.starts_with("-") {
            for char in chunk[1..].chars() {
                let index = available_flags
                    .iter()
                    .position(|&x| x.to_string() == char.to_string())
                    .unwrap();
                flags.push(format!("flag: {}", available_flags[index + 1].to_string()));
            }
        } else {
            flags.push(format!("parameter: {}", chunk.to_string()));
        }
    }
    flags
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_flags() {
        let available_flags = vec![
            "a",
            "all",
            "f",
            "force",
            "n",
            "networking",
            "N",
            "numerical-list",
        ];
        let input = "-aN 12 --verbose 192.168.0.44";
        let expected = vec![
            "flag: all",
            "flag: numerical-list",
            "parameter: 12",
            "flag: verbose",
            "parameter: 192.168.0.44",
        ];
        assert_eq!(expected, parse_flags(available_flags, input));
    }
}
