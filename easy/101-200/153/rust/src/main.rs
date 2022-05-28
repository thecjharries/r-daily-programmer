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

use cached::proc_macro::cached;

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

#[cached]
fn factorial(n: u64) -> u64 {
    if n < 2 {
        return 1;
    }
    n * factorial(n - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial() {
        assert_eq!(1, factorial(0));
        assert_eq!(1, factorial(1));
        assert_eq!(2, factorial(2));
        assert_eq!(120, factorial(5))
    }
}
