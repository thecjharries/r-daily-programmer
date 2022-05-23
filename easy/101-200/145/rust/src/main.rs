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

fn draw_fir_tree(max_width: usize, base_char: char, tree_char: char) -> String {
    let width = if 0 == max_width % 2 {
        max_width + 1
    } else {
        max_width
    };
    let mut result = String::new();
    for leaf_count in (1..width + 1).step_by(2) {
        result.push_str(" ".repeat((width - leaf_count) / 2).as_str());
        result.push_str(format!("{}", tree_char).repeat(leaf_count).as_str());
        result.push_str(" ".repeat((width - leaf_count) / 2).as_str());
        result.push('\n');
    }
    result.push_str(" ".repeat((width - 3) / 2).as_str());
    result.push_str(format!("{}", base_char).repeat(3).as_str());
    result.push_str(" ".repeat((width - 3) / 2).as_str());
    result.push('\n');
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(draw_fir_tree(3, '#', '*'), " * \n***\n###\n".to_string());
        assert_eq!(
            draw_fir_tree(4, '#', '*'),
            "  *  \n *** \n*****\n ### \n".to_string()
        );
        assert_eq!(
            draw_fir_tree(5, '#', '*'),
            "  *  \n *** \n*****\n ### \n".to_string()
        );
    }
}
