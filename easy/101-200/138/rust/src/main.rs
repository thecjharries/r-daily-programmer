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

    fn repulsion_force_from(&self, other: &Particle) -> f64 {
        ((self.mass * other.mass) / self.distance_from(other).powi(2) * 10000.0).round() / 10000.0
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

    #[test]
    fn test_particle_repulsion_force_from() {
        let p1 = Particle {
            mass: 1.0,
            position: (-5.2, 3.8),
        };
        let p2 = Particle {
            mass: 1.0,
            position: (8.7, -4.1),
        };
        assert_eq!(p1.repulsion_force_from(&p2), 0.0039);
        let p3 = Particle {
            mass: 4.0,
            position: (0.04, -0.02),
        };
        let p4 = Particle {
            mass: 4.0,
            position: (-0.02, -0.03),
        };
        assert_eq!(p3.repulsion_force_from(&p4), 4324.3243);
    }
}
