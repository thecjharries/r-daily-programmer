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

fn find_kaprekar_numbers(min: u32, max: u32) -> Vec<u32> {
    todo!()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_kaprekar_numbers() {
        assert_eq!(vec![1, 9, 45], find_kaprekar_numbers(1, 50));
        assert_eq!(vec![9, 45, 55, 99], find_kaprekar_numbers(2, 100));
        assert_eq!(
            vec![297, 703, 999, 2223, 2728, 4879, 4950, 5050, 5292, 7272, 7777],
            find_kaprekar_numbers(101, 9000)
        );
    }
}
