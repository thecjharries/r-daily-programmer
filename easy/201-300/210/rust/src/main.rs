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

fn build_compability(first: u32, second: u32) -> (f32, u32, u32) {
    let first_binary = format!("{:032b}", first).chars().collect::<Vec<_>>();
    let second_binary = format!("{:032b}", second).chars().collect::<Vec<_>>();
    let mut compability_count = 0;
    for index in 0..first_binary.len() {
        if first_binary[index] == second_binary[index] {
            compability_count += 1;
        }
    }
    let first_complement_binary: String = first_binary
        .iter()
        .map(|x| if *x == '1' { '0' } else { '1' })
        .collect();
    let second_complement_binary: String = second_binary
        .iter()
        .map(|x| if *x == '1' { '0' } else { '1' })
        .collect();
    (
        100.0 * (compability_count as f32 / first_binary.len() as f32),
        u32::from_str_radix(&first_complement_binary, 2).unwrap(),
        u32::from_str_radix(&second_complement_binary, 2).unwrap(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_compability() {
        assert_eq!((50.0, 4294967275, 4294901780), build_compability(20, 65515));
    }
}
