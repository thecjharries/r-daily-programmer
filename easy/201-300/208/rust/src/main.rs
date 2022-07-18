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

fn remove_duplicates(input: Vec<u32>) -> Vec<u32> {
    input.into_iter().fold(Vec::new(), |mut acc, x| {
        if !acc.contains(&x) {
            acc.push(x);
        }
        acc
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_duplicates() {
        assert_eq!(
            vec![3, 1, 4, 5, 2],
            remove_duplicates(vec![
                3, 1, 3, 4, 4, 1, 4, 5, 2, 1, 4, 4, 4, 4, 1, 4, 3, 2, 5, 5, 2, 2, 2, 4, 2, 4, 4, 4,
                4, 1
            ])
        );
        assert_eq!(vec![1, 2, 3, 4, 5], remove_duplicates(vec![1, 2, 3, 4, 5]));
        assert_eq!(
            vec![1, 2, 3, 4],
            remove_duplicates(vec![1, 1, 2, 2, 3, 3, 4, 4])
        );
    }
}
