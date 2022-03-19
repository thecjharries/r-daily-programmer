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

fn main() {
    println!("rad");
}

fn step_count(start: f64, end: f64, steps: i64) -> Vec<f64> {
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step_count() {
        assert_eq!(
            step_count(18.75, -22.00, 5),
            vec![18.75, 8.5625, -1.625, -11.8125, -22.0]
        );
        assert_eq!(
            step_count(-5.75, 12.00, 5),
            vec![-5.75, -1.3125, 3.125, 7.5625, 12.0]
        );
        assert_eq!(step_count(13.50, -20.75, 3), vec![13.5, -3.625, -20.75]);
        assert_eq!(
            step_count(9.75, 3.00, 9),
            vec![9.75, 8.90625, 8.0625, 7.21875, 6.375, 5.53125, 4.6875, 3.84375, 3.0]
        );
    }
}
