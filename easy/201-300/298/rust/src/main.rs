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

fn reduce_parentheses(input: &str) -> String {
    let mut output = String::from(input);
    let mut open_parentheses = Vec::new();
    let mut remove_indices: Vec<usize> = Vec::new();
    let mut previous_open: i32 = -1;
    let mut previous_close: i32 = -1;
    for (index, character) in input.chars().enumerate() {
        if '(' == character {
            open_parentheses.push(index);
        }
        if ')' == character {
            if let Some(open_parenthesis) = open_parentheses.last() {
                if 1 == index - open_parenthesis {
                    remove_indices.push(*open_parenthesis);
                    remove_indices.push(index);
                } else if *open_parenthesis == previous_open as usize - 1
                    && 1 == index - previous_close as usize
                {
                    remove_indices.push(*open_parenthesis);
                    remove_indices.push(index);
                }
                previous_close = index as i32;
                previous_open = open_parentheses.pop().unwrap_or(0) as i32;
            }
        }
    }
    for index in remove_indices {
        output.replace_range(index..index + 1, " ");
    }
    output.replace(" ", "")
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!("((a((bc)(de)))f)", reduce_parentheses("((a((bc)(de)))f)"));
        assert_eq!("((zbcd)((e)fg))", reduce_parentheses("(((zbcd)(((e)fg))))"));
        assert_eq!("ab(c)", reduce_parentheses("ab((c))"));
        assert_eq!("", reduce_parentheses("()"));
        assert_eq!("(fgh)", reduce_parentheses("((fgh()()()))"));
        assert_eq!("(abc)", reduce_parentheses("()(abc())"));
    }
}
