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

fn format_in_columns(
    number_of_columns: usize,
    width_of_columns: usize,
    column_padding: usize,
    text: &str,
) -> String {
    let words = text.split_whitespace();
    let mut lines = Vec::new();
    let mut current_line = String::new();
    for word in words {
        if current_line.len() + word.len() >= width_of_columns {
            current_line.push_str(" ".repeat(width_of_columns - current_line.len()).as_str());
            lines.push(current_line);
            current_line = String::new();
        }
        if !current_line.is_empty() {
            current_line.push(' ');
        }
        current_line.push_str(word);
    }
    if !current_line.is_empty() {
        current_line.push_str(" ".repeat(width_of_columns - current_line.len()).as_str());
        lines.push(current_line);
    }
    println!("{:?}", lines);
    let mut result = String::new();
    let lines_per_column = (lines.len() as f32 / number_of_columns as f32).ceil() as usize;
    for line_index in 0..lines_per_column {
        let mut current_line = Vec::new();
        for column_index in 0..number_of_columns {
            if column_index * lines_per_column + line_index < lines.len() {
                current_line.push(lines[column_index * lines_per_column + line_index].to_string());
            } else {
                current_line.push(" ".repeat(width_of_columns));
            }
        }
        result.push_str(&current_line.join(" ".repeat(column_padding).as_str()));
        result.push('\n');
    }
    result.pop();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_in_columns() {
        assert_eq!(
            "Lorem ipsum dolor sit         adipiscing elit. Ut at        sodales ipsum. Vivamus   \namet, consectetur             pharetra sapien, id                                    ",
            format_in_columns(3, 25, 5, "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Ut at pharetra sapien, id sodales ipsum. Vivamus")
        );
    }
}
