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

const DIGITS_PRECISION: i32 = 6;

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn sqrt(number: f32) -> f32 {
    let mut low = 0.0;
    let mut high = number;
    let mut guess = (low + high) / 2.0;
    while (guess * guess - number).abs() > 10.0_f32.powi(-DIGITS_PRECISION) {
        if guess * guess > number {
            high = guess;
        } else {
            low = guess;
        }
        guess = (low + high) / 2.0;
    }
    (guess * 10.0_f32.powi(DIGITS_PRECISION)).round() / 10.0_f32.powi(DIGITS_PRECISION)
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sqrt() {
        assert_eq!(2.0, sqrt(4.0));
        assert_eq!(3.0, sqrt(9.0));
        assert_eq!(4.0, sqrt(16.0));
        assert_eq!(2.236068, sqrt(5.0));
        assert_eq!(3.162278, sqrt(10.0));
    }
}
