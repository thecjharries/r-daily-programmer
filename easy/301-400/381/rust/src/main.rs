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

fn yahtzee_upper(numbers: Vec<u32>) -> u32 {
    todo!()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(10, yahtzee_upper(vec![2, 3, 5, 5, 6]));
        assert_eq!(4, yahtzee_upper(vec![1, 1, 1, 1, 3]));
        assert_eq!(6, yahtzee_upper(vec![1, 1, 1, 3, 3]));
        assert_eq!(5, yahtzee_upper(vec![1, 2, 3, 4, 5]));
        assert_eq!(30, yahtzee_upper(vec![6, 6, 6, 6, 6]));
        assert_eq!(
            123456,
            yahtzee_upper(vec![
                1654, 1654, 50995, 30864, 1654, 50995, 22747, 1654, 1654, 1654, 1654, 1654, 30864,
                4868, 1654, 4868, 1654, 30864, 4868, 30864
            ])
        );
    }
}
