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

fn draw_fir_tree(max_width: u16, base_char: char, tree_char: char) -> String {
    String::new()
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
