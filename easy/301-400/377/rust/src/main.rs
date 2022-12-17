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

fn fit1(crate_width: u64, crate_height: u64, box_width: u64, box_height: u64) -> u64 {
    (crate_width / box_width) * (crate_height / box_height)
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(12, fit1(25, 18, 6, 5));
        assert_eq!(100, fit1(10, 10, 1, 1));
        assert_eq!(10, fit1(12, 34, 5, 6));
        assert_eq!(5676, fit1(12345, 678910, 1112, 1314));
        assert_eq!(0, fit1(5, 100, 6, 1));
    }
}
