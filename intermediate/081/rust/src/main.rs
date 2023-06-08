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

use rand::distributions::Uniform;
use rand::prelude::*;
use rand::Rng;
use rand_pcg::Pcg64;

const MAXIMUM_ITERATIONS: usize = 100000;

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

// https://old.reddit.com/r/dailyprogrammer/comments/x539t/7252012_challenge_81_intermediate_local/c5ja7xb/
fn find_minimum<R: Rng>(f: &dyn Fn(Vec<f32>) -> f32, origin: Vec<f32>, rng: &mut R) -> Vec<f32> {
    let f_0 = f(origin.clone());
    let mut minimum = origin.clone();
    let uniform = Uniform::new(-1.0, 1.0);
    for index in 0..MAXIMUM_ITERATIONS {
        let radius = (-1.0 * index as f32).exp();
        let new_origin = origin
            .iter()
            .map(|x| x + radius * uniform.sample(rng))
            .collect::<Vec<f32>>();
        let f_new = f(new_origin.clone());
        if f_new < f_0 {
            minimum = new_origin;
        }
    }
    minimum
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_minimum() {
        let mut rng = Pcg64::seed_from_u64(0);
        let origin = vec![0.0, 0.0];
        let result = find_minimum(
            &|x: Vec<f32>| x[0] * x[0] + x[1] * x[1],
            origin.clone(),
            &mut rng,
        );
        assert_eq!(0.0, result[0]);
        assert_eq!(0.0, result[1]);
        let mut rng = Pcg64::seed_from_u64(0);
        let origin = vec![0.0, 0.0, 0.0];
        let result = find_minimum(
            &|x: Vec<f32>| x[0] * x[0] + x[1] * x[1] + x[2] * x[2],
            origin.clone(),
            &mut rng,
        );
        assert_eq!(0.0, result[0]);
        assert_eq!(0.0, result[1]);
        assert_eq!(0.0, result[2]);
    }
}
