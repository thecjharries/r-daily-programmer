// Copyright 2023 CJ Harries
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

fn find_perfect_power(number: u32) -> (u32, u32) {
    let mut base = 2;
    let mut result = u32::MAX;
    while 2 < result {
        let mut result_float = (number as f64).log(base as f64);
        result = result_float.ceil() as u32;
        if result_float.floor() == result_float {
            return (base, result);
        }
        base += 1;
    }
    (number, 1)
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_perfect_power() {
        assert_eq!((17, 1), find_perfect_power(17));
        assert_eq!((2, 30), find_perfect_power(1073741824));
        assert_eq!((5, 2), find_perfect_power(25));
    }
}
