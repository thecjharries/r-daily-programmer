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

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn plot_y_equals_x(width: usize) -> Vec<String> {
    let mut output = Vec::new();
    for row in 0..width {
        let mut row_string = String::new();
        for column in 0..width {
            if row == column {
                row_string.push_str("X");
            } else {
                row_string.push_str(" ");
            }
        }
        output.push(row_string.chars().rev().collect());
    }
    output
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plot_y_equals_x() {
        assert_eq!(vec!["  X", " X ", "X  "], plot_y_equals_x(3));
    }
}
