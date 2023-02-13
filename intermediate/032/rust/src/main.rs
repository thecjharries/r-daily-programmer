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

fn compute_hanoi_moves(disk_count: u32) -> u32 {
    2_u32.pow(disk_count) - 1
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_hanoi_moves() {
        assert_eq!(1, compute_hanoi_moves(1));
        assert_eq!(3, compute_hanoi_moves(2));
        assert_eq!(7, compute_hanoi_moves(3));
        assert_eq!(15, compute_hanoi_moves(4));
        assert_eq!(31, compute_hanoi_moves(5));
    }
}
