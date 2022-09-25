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
fn insert_weave<T: Clone>(first: Vec<T>, second: Vec<T>) -> Vec<T> {
    let mut result = Vec::new();
    let mut first_index = 0;
    for second_index in 0..second.len() - 1 {
        result.push(second[second_index].clone());
        result.push(first[first_index].clone());
        first_index += 1;
        first_index %= first.len();
    }
    result.push(second[second.len() - 1].clone());
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_weave() {
        assert_eq!(
            vec![0, 11, 1, 11, 2, 11, 3],
            insert_weave(vec![11], vec![0, 1, 2, 3])
        );
        assert_eq!(
            vec![0, 11, 1, 12, 2, 11, 3],
            insert_weave(vec![11, 12], vec![0, 1, 2, 3])
        );
        assert_eq!(vec![0, 11, 1], insert_weave(vec![11, 12, 13], vec![0, 1]));
    }
}
