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

use std::io::Write;

fn main() {
    println!("rad");
}

fn fizz_buzz(max: i32, stdout: &mut dyn Write) {
    for index in 1..max+1 {
        let mut line = String::new();
        if 0 == index%3 {
            line.push_str("Fizz");
        }
        if 0 == index%5 {
            line.push_str("Buzz");
        }
        if 0 == line.len() {
            line.push_str(&index.to_string());
        }
        writeln!(stdout, "{}", line).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fizz_buzz() {
        let mut stdout = Vec::new();
        let max = 16;
        fizz_buzz(max, &mut stdout);
        assert_eq!(stdout, b"1\n2\nFizz\n4\nBuzz\nFizz\n7\n8\nFizz\nBuzz\n11\nFizz\n13\n14\nFizzBuzz\n16\n");
    }
}
