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

fn sieve_of_sundaram(max: u32) -> Vec<u32> {
    todo!()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(vec![2, 3, 5, 7], sieve_of_sundaram(10));
        assert_eq!(
            vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29],
            sieve_of_sundaram(30)
        );
        assert_eq!(1229, sieve_of_sundaram(10000).len());
    }
}
