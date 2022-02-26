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

const DIGITS: str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn main() {
    println!("rad");
}

fn convert_number_to_base(number: i32, base: usize) -> String {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_number_to_base() {
        assert_eq!(convert_number_to_base(0, 2), "0");
        assert_eq!(convert_number_to_base(1, 2), "1");
        assert_eq!(convert_number_to_base(2, 2), "10");
        assert_eq!(convert_number_to_base(10, 36), "A");
        assert_eq!(convert_number_to_base(12345678, 23), "1L2FHE");
        assert_eq!(convert_number_to_base(12345678, 19), "4IDHAA");
    }
}
