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

fn nonogram_row(row: Vec<u8>) -> Vec<u8> {
    let mut result = Vec::new();
    let mut current_count = 0;
    for cell in row {
        if 0 == cell {
            if 0 < current_count {
                result.push(current_count);
                current_count = 0;
            }
        } else {
            current_count += 1;
        }
    }
    if 0 < current_count {
        result.push(current_count);
    }
    result
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(vec![] as Vec<u8>, nonogram_row(vec![]));
        assert_eq!(vec![] as Vec<u8>, nonogram_row(vec![0, 0, 0, 0, 0]));
        assert_eq!(vec![5], nonogram_row(vec![1, 1, 1, 1, 1]));
        assert_eq!(
            vec![5, 4],
            nonogram_row(vec![0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1])
        );
        assert_eq!(
            vec![2, 1, 3],
            nonogram_row(vec![1, 1, 0, 1, 0, 0, 1, 1, 1, 0, 0])
        );
        assert_eq!(
            vec![2, 1, 3],
            nonogram_row(vec![0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 1, 1, 1])
        );
        assert_eq!(
            vec![1, 1, 1, 1, 1, 1, 1, 1],
            nonogram_row(vec![1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1])
        );
    }
}
