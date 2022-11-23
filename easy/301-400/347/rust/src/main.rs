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

fn compute_light_time(visitor_times: Vec<(u8, u8)>) -> u8 {
    todo!()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(3, compute_light_time(vec![(1, 3), (2, 3), (4, 5)]));
        assert_eq!(7, compute_light_time(vec![(2, 4), (3, 6), (1, 3), (6, 8)]));
        assert_eq!(
            5,
            compute_light_time(vec![(6, 8), (5, 8), (8, 9), (5, 7), (4, 7)])
        );
    }
}
