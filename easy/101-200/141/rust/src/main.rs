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

fn compute_fletcher16_checksum(data: &[u8]) -> u16 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_fletcher16_checksum() {
        assert_eq!(compute_fletcher16_checksum(b"Fletcher"), 0xD330);
        assert_eq!(
            compute_fletcher16_checksum(b"Sally sells seashells by the seashore."),
            0xD23E
        );
        assert_eq!(
            compute_fletcher16_checksum(
                b"Les chaussettes de l'archi-duchesse, sont-elles seches ou archi-seches ?"
            ),
            0x404D
        );
    }
}
