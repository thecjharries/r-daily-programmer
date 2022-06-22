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

fn get_first_n_look_and_say_iterations(iterations: u32, seed: u32) -> Vec<u32> {
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_first_n_look_and_say_iterations() {
        assert_eq!(
            vec![1, 11, 21, 1211, 111221, 312211, 13112221],
            get_first_n_look_and_say_iterations(7, 1)
        );
        assert_eq!(
            vec![22, 22, 22, 22, 22, 22, 22, 22],
            get_first_n_look_and_say_iterations(8, 22)
        )
    }
}
