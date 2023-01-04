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

#[derive(Debug, PartialEq)]
struct RealVector(Vec<f64>);

impl RealVector {
    pub fn round_to_five_places(input: f64) -> f64 {
        (input * 100000.0).round() / 100000.0
    }

    pub fn length(&self) -> f64 {
        RealVector::round_to_five_places(
            self.0
                .iter()
                .fold(0.0, |accumulator, value| accumulator + value.powi(2))
                .sqrt(),
        )
    }

    pub fn normalize(&self) -> RealVector {
        let length = self
            .0
            .iter()
            .fold(0.0, |accumulator, value| accumulator + value.powi(2))
            .sqrt();
        RealVector(
            self.0
                .iter()
                .map(|value| RealVector::round_to_five_places(value / length))
                .collect(),
        )
    }

    pub fn dot_product(&self, other: &RealVector) -> f64 {
        self.0
            .iter()
            .zip(other.0.iter())
            .fold(0.0, |accumulator, (left, right)| accumulator + left * right)
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
    fn test_round_to_five_places() {
        assert_eq!(0.0, RealVector::round_to_five_places(0.0));
        assert_eq!(1.00001, RealVector::round_to_five_places(1.000009));
    }

    #[test]
    fn test_length() {
        let test_vector = RealVector(vec![1.0, 1.0]);
        assert_eq!(1.41421, test_vector.length());
    }

    #[test]
    fn test_normalize() {
        let input = RealVector(vec![1.2, 3.4]);
        let output = RealVector(vec![0.33282, 0.94299]);
        assert_eq!(output, input.normalize());
    }

    #[test]
    fn test_dot_product() {
        let left = RealVector(vec![1.0, 2.0]);
        let right = RealVector(vec![3.0, 4.0]);
        assert_eq!(11.0, left.dot_product(&right));
    }
}
