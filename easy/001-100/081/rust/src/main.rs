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

// Note that a derivative is not defined at an endpoint
// This means we can have at most y_range.len() - 2 points
fn derivative(x_min: f64, x_max: f64, y_range: Vec<f64>) -> Vec<f64> {
    let x_bump = (x_max - x_min) / (y_range.len() - 1) as f64;
    let mut slopes = Vec::new();
    for index in 1..y_range.len() - 1 {
        slopes.push((y_range[index + 1] - y_range[index - 1]) / (2.0 * x_bump));
    }
    slopes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derivative() {
        assert_eq!(
            derivate(-1.0, 1.0, vec![-1.0,-.5,0,.5,1.0]),
            vec![1.0, 1.0, 1.0]
        );
    }
}
