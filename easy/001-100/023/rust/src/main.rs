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

fn split_in_half<T>(input: &[T]) -> (&[T], &[T]) {
    let len = input.len();
    let mid = len / 2;
    (&input[..mid], &input[mid..])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_in_half() {
        assert_eq!(split_in_half(&[1, 2, 3, 4, 5]), (&[1, 2], &[3, 4, 5]));
        assert_eq!(split_in_half(&["a", "b", "c"]), (&["a"], &["b", "c"]));
    }
}
