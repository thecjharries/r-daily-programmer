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

fn rgb_to_grayscale(red: u8, green: u8, blue: u8) -> u8 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb_to_grayscale() {
        assert_eq!(0, rgb_to_grayscale(0, 0, 0));
        assert_eq!(255, rgb_to_grayscale(255, 255, 255));
        assert_eq!(76, rgb_to_grayscale(255, 0, 0));
        assert_eq!(150, rgb_to_grayscale(0, 255, 0));
        assert_eq!(28, rgb_to_grayscale(0, 0, 255));
    }
}
