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

fn make_checkerboard(width: u32, height: u32) -> Vec<String> {
    let mut checkerboard = Vec::new();
    for y in 0..height {
        for _ in 0..3 {
            let mut row = String::new();
            for x in 0..width {
                if x % 2 != y % 2 {
                    row.push('#');
                    row.push('#');
                    row.push('#');
                } else {
                    row.push(' ');
                    row.push(' ');
                    row.push(' ');
                }
            }
            checkerboard.push(row);
        }
    }
    checkerboard
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_checkerboard() {
        let output = vec![
            "   ###   ###   ".to_string(),
            "   ###   ###   ".to_string(),
            "   ###   ###   ".to_string(),
            "###   ###   ###".to_string(),
            "###   ###   ###".to_string(),
            "###   ###   ###".to_string(),
        ];
        assert_eq!(make_checkerboard(5,2), output);
    }
}
