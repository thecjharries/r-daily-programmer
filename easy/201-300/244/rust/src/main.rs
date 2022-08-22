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

fn fork_fn(
    first: &dyn Fn(Vec<u32>) -> u32,
    second: &dyn Fn(Vec<u32>) -> u32,
    remaining: Vec<&dyn Fn(Vec<u32>) -> u32>,
) -> Fn(Vec<u32>) -> u32 {
    if 1 == remaining.len() {
        fn single(input: Vec<u32>) -> u32 {
            second(vec![first(input), remaining[0](input)])
        }
        single
    } else if 1 == remaining.len() % 2 {
        fn multiple(input: Vec<u32>) -> u32 {
            second(vec![
                first(input),
                fork_fn(remaining[0], remaining[1], remaining[2..].to_vec())(input),
            ])
        }
        multiple
    } else {
        panic!("no functions to combine");
    }
    fn failure() -> u32 {
        0
    }
    failure
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        fn sum(input: Vec<u32>) -> u32 {
            input.iter().sum()
        }
        fn count(input: Vec<u32>) -> u32 {
            input.len() as u32
        }
        fn divide(input: Vec<u32>) -> u32 {
            input[0] / input[1]
        }
        let first = fork_fn(&sum, &divide, vec![&count]);
        assert_eq!(3, first(vec![1, 2, 3, 4, 5]));
        let second = fork_fn(&sum, &divide, vec![&sum, &divide, &count]);
        assert_eq!(5, second(vec![1, 2, 3, 4, 5]));
        assert_eq!(0, fork_fn(&sum, &divide, vec![])(vec![1, 2, 3, 4, 5]));
    }
}
