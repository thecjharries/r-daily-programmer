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

static CAPS: [f64; 3] = [10000.0, 30000.0, 100000.0];
static RATES: [f64; 4] = [0.0, 0.1, 0.25, 0.4];

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn tax(amount: f64) -> f64 {
    todo!()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(tax(0.0), 0.0);
        assert_eq!(tax(10000.0), 0.0);
        assert_eq!(tax(10009.0), 0.0);
        assert_eq!(tax(10010.0), 1.0);
        assert_eq!(tax(12000.0), 200.0);
        assert_eq!(tax(56789.0), 8697.0);
        assert_eq!(tax(1234567.0), 473326.0);
    }
}
