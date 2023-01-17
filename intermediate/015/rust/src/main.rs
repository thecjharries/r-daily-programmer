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

use rand::prelude::SliceRandom;
use rand::thread_rng;

const MIN: usize = 0;
const MAX: usize = 29;

struct Flea {
    x: usize,
    y: usize,
}

impl Flea {
    fn new(x: usize, y: usize) -> Flea {
        Flea { x, y }
    }

    fn jump(&mut self) {
        let mut x_moves = Vec::new();
        let mut y_moves = Vec::new();
        if self.x > MIN {
            x_moves.push(self.x - 1);
        }
        if self.x < MAX {
            x_moves.push(self.x + 1);
        }
        if self.y > MIN {
            y_moves.push(self.y - 1);
        }
        if self.y < MAX {
            y_moves.push(self.y + 1);
        }
        let mut rng = thread_rng();
        self.x = *x_moves.choose(&mut rng).unwrap();
        self.y = *y_moves.choose(&mut rng).unwrap();
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
    fn test_flea_new() {
        let flea = Flea::new(0, 0);
        assert_eq!(flea.x, 0);
        assert_eq!(flea.y, 0);
    }

    #[test]
    fn test_flea_jump() {
        let mut flea = Flea::new(MIN, MIN);
        flea.jump();
        assert!(flea.x > MIN && flea.x <= MAX);
        assert!(flea.y > MIN && flea.y <= MAX);
        flea = Flea::new(MAX, MAX);
        flea.jump();
        assert!(flea.x >= MIN && flea.x < MAX);
        assert!(flea.y >= MIN && flea.y < MAX);
    }
}
