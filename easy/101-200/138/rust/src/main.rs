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

struct Particle {
    mass: f64,
    position: (f64, f64),
}

impl Particle {
    fn distance_from(&self, other: &Particle) -> f64 {
        ((self.position.0 - other.position.0).powi(2)
            + (self.position.1 - other.position.1).powi(2))
        .sqrt()
    }
}

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_particle_distance_from() {
        let p1 = Particle {
            mass: 1.0,
            position: (1.0, 1.0),
        };
        let p2 = Particle {
            mass: 1.0,
            position: (2.0, 2.0),
        };
        assert_eq!(p1.distance_from(&p2), 2.0_f64.sqrt());
    }
}
