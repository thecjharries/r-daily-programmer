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

#[derive(Debug, PartialEq, Clone)]
struct NRealNumber(Vec<f64>);

impl NRealNumber {
    fn new(number: Vec<f64>) -> Self {
        Self(number)
    }

    fn length(&self) -> f64 {
        self.0
            .clone()
            .iter_mut()
            .map(|number| number.powi(2))
            .sum::<f64>()
            .sqrt()
    }
}

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nrealnumber_new_creates_a_new_vec() {
        assert_eq!(
            NRealNumber(vec![1.0, 2.0, 3.0]),
            NRealNumber::new(vec![1.0, 2.0, 3.0])
        );
    }

    #[test]
    fn nrealnumber_length_gives_euclidean_length() {
        assert_eq!(5.0, NRealNumber::new(vec![3.0, 4.0]).length());
    }
}
