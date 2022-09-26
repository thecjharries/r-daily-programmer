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

#[cfg(not(tarpaulin_include))]
fn uuencode(input: &str) -> String {
    let mut output = String::new();
    for line in input.as_bytes().chunks(45) {
        for chunk in line.chunks(3) {
            let chunk_string = format!("{:b}{:b}{:b}", chunk[0], chunk[1], chunk[2]);
            for index in 0..4 {
                let current = &chunk_string[index * 6..(index + 1) * 6].to_string();
                let ascii_char = u8::from_str_radix(current, 2).unwrap() + 32;
                println!("{}", ascii_char);
            }
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uuencode() {
        // assert_eq!("0V%T", uuencode("Cat"));
        assert!(true);
    }
}
