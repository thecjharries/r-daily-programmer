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

fn qcheck(places: Vec<u8>) -> bool {
    for (first_index, first_place) in places.iter().enumerate() {
        for (second_index, second_place) in places.iter().enumerate() {
            if first_index == second_index {
                continue;
            }
            if first_place == second_place {
                return false;
            }
            if 1.0
                == ((*first_place as f64 - *second_place as f64)
                    / (first_index as f64 - second_index as f64))
                    .abs()
            {
                return false;
            }
        }
    }
    true
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(true, qcheck(vec![4, 2, 7, 3, 6, 8, 5, 1]));
        assert_eq!(true, qcheck(vec![2, 5, 7, 4, 1, 8, 6, 3]));
        assert_eq!(false, qcheck(vec![5, 3, 1, 4, 2, 8, 6, 3]));
        assert_eq!(false, qcheck(vec![5, 8, 2, 4, 7, 1, 3, 6]));
        assert_eq!(false, qcheck(vec![4, 3, 1, 8, 1, 3, 5, 2]));
    }
}
