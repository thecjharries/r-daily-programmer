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
    registers: BTreeMap<u8, bool>,
    program_counter: usize,
}

impl Simulation {
    fn new() -> Self {
        let mut registers = BTreeMap::new();
        for index in 0..32 {
            registers.insert(index, false);
        }
        Self {
            registers,
            program_counter: 0,
        }
    }

    fn and(&mut self, a: u8, b: u8) {
        self.registers
            .insert(a, self.registers[&a] & self.registers[&b]);
    }

    fn or(&mut self, a: u8, b: u8) {
        self.registers
            .insert(a, self.registers[&a] | self.registers[&b]);
    }

    fn xor(&mut self, a: u8, b: u8) {
        self.registers
            .insert(a, self.registers[&a] ^ self.registers[&b]);
    }

    fn not(&mut self, a: u8) {
        self.registers.insert(a, !self.registers[&a]);
    }

    fn set(&mut self, a: u8, value: u8) {
        self.registers.insert(a, 0 != value);
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
        assert_eq!(false, simulation.registers[&31]);
    }

    #[test]
    fn simulation_can_perform_binary_operations() {
        let mut simulation = Simulation::new();
        simulation.registers.insert(0, true);
        simulation.registers.insert(1, false);
        simulation.and(2, 0);
        assert_eq!(false, simulation.registers[&2]);
        simulation.or(2, 0);
        assert_eq!(true, simulation.registers[&2]);
        simulation.xor(2, 0);
        assert_eq!(false, simulation.registers[&2]);
        simulation.not(1);
        assert_eq!(true, simulation.registers[&1]);
    }

    #[test]
    fn simulation_set() {
        let mut simulation = Simulation::new();
        simulation.set(0, 1);
        assert_eq!(true, simulation.registers[&0]);
        simulation.set(0, 0);
        assert_eq!(false, simulation.registers[&0]);
    }
}
