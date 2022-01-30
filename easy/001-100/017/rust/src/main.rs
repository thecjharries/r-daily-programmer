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

fn create_triangle(height: usize, character: char) -> String {
    let mut triangle: Vec<String> = Vec::new();
    for row in 0..height {
        triangle.push(character.to_string().repeat(usize::pow(2, row as u32)));
    }
    triangle.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_triangle() {
        assert_eq!(create_triangle(1, '*'), "*");
        assert_eq!(create_triangle(2, '*'), "*\n**");
        assert_eq!(create_triangle(3, '*'), "*\n**\n****");
        assert_eq!(create_triangle(4, '*'), "*\n**\n****\n********");
    }
}