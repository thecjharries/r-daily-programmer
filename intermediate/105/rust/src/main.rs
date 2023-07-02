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

fn basic_bool_eval(input: &str) -> u8 {
    let mut digit_stack = Vec::new();
    let mut operator_stack = Vec::new();
    for character in input.chars() {
        match character {
            '0' => digit_stack.push(0),
            '1' => digit_stack.push(1),
            ' ' => continue,
            _ => {
                operator_stack.push(character);
            }
        }
        if 2 == digit_stack.len() {
            let operator = operator_stack.pop().unwrap();
            let right = digit_stack.pop().unwrap();
            let left = digit_stack.pop().unwrap();
            digit_stack.push(match operator {
                '*' => left & right,
                '|' => left | right,
                '^' => left ^ right,
                _ => unreachable!(),
            });
        }
    }
    if let Some(operator) = operator_stack.pop() {
        let right = digit_stack.pop().unwrap();
        digit_stack.push(match operator {
            '!' => {
                if 0 == right {
                    1
                } else {
                    0
                }
            }
            _ => unreachable!(),
        });
    }
    digit_stack.pop().unwrap()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_bool_eval() {
        assert_eq!(1, basic_bool_eval("1"));
        assert_eq!(0, basic_bool_eval("0"));
        assert_eq!(1, basic_bool_eval("1 * 1"));
        assert_eq!(0, basic_bool_eval("1 * 0"));
        assert_eq!(0, basic_bool_eval("0 * 1"));
        assert_eq!(0, basic_bool_eval("0 * 0"));
        assert_eq!(1, basic_bool_eval("1 | 1"));
        assert_eq!(1, basic_bool_eval("1 | 0"));
        assert_eq!(1, basic_bool_eval("0 | 1"));
        assert_eq!(0, basic_bool_eval("0 | 0"));
        assert_eq!(1, basic_bool_eval("1 ^ 0"));
        assert_eq!(1, basic_bool_eval("0 ^ 1"));
        assert_eq!(0, basic_bool_eval("1 ^ 1"));
        assert_eq!(0, basic_bool_eval("0 ^ 0"));
        assert_eq!(1, basic_bool_eval("!0"));
        assert_eq!(0, basic_bool_eval("!1"));
    }
}
