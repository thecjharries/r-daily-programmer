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

use memoize::memoize;

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

#[memoize]
fn find_complexity(n: u32) -> u32 {
    if n < 2 {
        return 1;
    }
    let mut complexity = find_complexity(n - 1) + 1;
    for index in 2..((n as f32).sqrt().floor() as u32 + 1) {
        if 0 == n % index {
            let possible = find_complexity(index) + find_complexity(n / index);
            if possible < complexity {
                complexity = possible;
            }
        }
    }
    complexity
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(1, find_complexity(1));
        assert_eq!(2, find_complexity(2));
        assert_eq!(3, find_complexity(3));
        assert_eq!(4, find_complexity(4));
        assert_eq!(5, find_complexity(5));
        assert_eq!(5, find_complexity(6));
        assert_eq!(6, find_complexity(7));
        assert_eq!(6, find_complexity(8));
        assert_eq!(6, find_complexity(9));
        assert_eq!(7, find_complexity(10));
        assert_eq!(8, find_complexity(11));
        assert_eq!(7, find_complexity(12));
        assert_eq!(27, find_complexity(6161));
    }
}
