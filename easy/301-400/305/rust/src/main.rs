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

fn int_to_permbase2(input: u64) -> String {
    let mut current = input;
    let mut power = 1;
    while 2_u64.pow(power) <= current {
        current -= 2_u64.pow(power);
        power += 1
    }
    let representation = format!("{:b}", current);
    "0".repeat(power as usize - representation.len()) + &representation
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_int_to_permbase2() {
        assert_eq!("0101", int_to_permbase2(19));
        assert_eq!("11000", int_to_permbase2(54));
        assert_eq!("111000111", int_to_permbase2(965));
    }
}
