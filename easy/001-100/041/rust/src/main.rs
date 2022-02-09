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

use std::iter::repeat;

fn main() {
    println!("rad");
}

fn wrap_in_banner(input: &str) -> Vec<String> {
    let mut output = Vec::new();
    output.push(repeat("*").take(input.len()+6).collect::<String>());
    output.push("*".to_owned() + &repeat(" ").take(input.len()+4).collect::<String>() + &"*".to_owned());
    output.push("*  ".to_owned() + &input + &"  *".to_owned());
    output.push("*".to_owned() + &repeat(" ").take(input.len()+4).collect::<String>() + &"*".to_owned());
    output.push(repeat("*").take(input.len()+6).collect::<String>());
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wrap_in_banner() {
        let output = vec![
            "*****************************************".to_string(),
            "*                                       *".to_string(),
            "*  So long and thanks for all the fish  *".to_string(),
            "*                                       *".to_string(),
            "*****************************************".to_string(),
        ];
        assert_eq!(output, wrap_in_banner("So long and thanks for all the fish"));
    }
}
