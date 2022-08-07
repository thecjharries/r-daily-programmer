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

fn find_dottie_number() -> f32 {
    let mut current_number: f32 = 0.0;
    let mut next_number: f32 = 1.0;
    while (current_number - next_number).abs() > f32::EPSILON {
        current_number = next_number;
        next_number = current_number.cos();
    }
    current_number
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_dottie_number() {
        assert_eq!(0.7390851332151606, find_dottie_number());
    }
}
