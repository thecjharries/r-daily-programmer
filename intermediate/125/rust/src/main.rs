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

use std::collections::BTreeMap;

#[derive(Debug, PartialEq)]
struct Simulation {
    registers: BTreeMap<u8, u8>,
    program_counter: usize,
}

impl Simulation {
    fn new() -> Self {
        let mut registers = BTreeMap::new();
        for index in 0..32 {
            registers.insert(index, 0);
        }
        Self {
            registers,
            program_counter: 0,
        }
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
    fn simulation_new() {
        let simulation = Simulation::new();
        assert_eq!(32, simulation.registers.len());
        assert_eq!(0, simulation.program_counter);
        assert_eq!(0, simulation.registers[&31]);
    }
}
