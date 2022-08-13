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

fn get_distinct_prime_factors(number: u64) -> Vec<u64> {
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_distinct_prime_factors() {
        assert_eq!(vec![23], get_distinct_prime_factors(23));
        assert_eq!(vec![2,3], get_distinct_prime_factors(24))
        assert_eq!(vec![2, 3, 7, 17], get_distinct_prime_factors(714));
        assert_eq!(vec![5, 11, 13], get_distinct_prime_factors(715));
    }
}
