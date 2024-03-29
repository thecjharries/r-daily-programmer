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

fn get_derangement_count(size: u128) -> u128 {
    if 0 == size {
        return 1;
    }
    if 1 == size {
        return 0;
    }
    (size - 1) * (get_derangement_count(size - 1) + get_derangement_count(size - 2))
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(1, get_derangement_count(0));
        assert_eq!(0, get_derangement_count(1));
        assert_eq!(265, get_derangement_count(6));
        assert_eq!(133496, get_derangement_count(9));
        assert_eq!(32071101049, get_derangement_count(14));
    }
}
