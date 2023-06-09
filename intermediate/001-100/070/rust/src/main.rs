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
fn hyper(n: u128, a: u128, b: u128) -> u128 {
    if 0 == n {
        return b + 1;
    } else if 0 == b {
        if 1 == n {
            return a;
        } else if 2 == n {
            return 0;
        } else {
            return 1;
        }
    } else {
        return hyper(n - 1, a, hyper(n, a, b - 1));
    }
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hyper() {
        assert_eq!(2, hyper(0, 10, 1));
        assert_eq!(10, hyper(1, 10, 0));
        assert_eq!(0, hyper(2, 10, 0));
        assert_eq!(1, hyper(4, 10, 0));
        assert_eq!(10, hyper(2, 10, 1));
    }
}
