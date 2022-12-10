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

fn convert_to_hex(red: u8, green: u8, blue: u8) -> String {
    todo!()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(convert_to_hex(255, 99, 71), "#FF6347");
        assert_eq!(convert_to_hex(184, 134, 11), "#B8860B");
        assert_eq!(convert_to_hex(189, 183, 107), "#BDB76B");
        assert_eq!(convert_to_hex(0, 0, 205), "#0000CD");
    }
}
