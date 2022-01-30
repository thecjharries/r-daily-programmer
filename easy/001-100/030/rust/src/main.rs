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

fn can_sum(target: i32, numbers: Vec<i32>) -> (i32, i32) {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_sum() {
        assert_eq!(can_sum(6, vec![1, 2, 3, 4, 5]), (1, 5));
        assert_eq!(can_sum(10, vec![1, 2, 3, 4, 5]), (-1, -1));
    }
}
