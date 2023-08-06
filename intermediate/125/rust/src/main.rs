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

use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64;
use std::collections::BTreeMap;

#[derive(Debug, PartialEq)]
enum SimulationState {
    Created,
    Running,
    Halted,
    Ended,
}

#[derive(Debug, PartialEq)]
enum Instruction {
    And(u8, u8),
    Or(u8, u8),
    Xor(u8, u8),
    Not(u8),
    Set(u8, u8),
    Random(u8),
    Jump(usize),
    Jz(usize, u8),
    Halt,
}

#[derive(Debug, PartialEq)]
struct Simulation {
    registers: BTreeMap<u8, bool>,
    program_counter: usize,
    steps: usize,
    rng: Pcg64,
    state: SimulationState,
    instructions: Vec<Instruction>,
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
            steps: 0,
            rng: Pcg64::seed_from_u64(0),
            state: SimulationState::Created,
            instructions: Vec::new(),
        }
    }

    fn check_program_end(&mut self) {
        if self.program_counter >= self.instructions.len() {
            self.state = SimulationState::Ended;
        }
    }

    fn and(&mut self, a: u8, b: u8) {
        self.state = SimulationState::Running;
        self.registers
            .insert(a, self.registers[&a] & self.registers[&b]);
        self.program_counter += 1;
        self.steps += 1;
        self.check_program_end();
    }

    fn or(&mut self, a: u8, b: u8) {
        self.state = SimulationState::Running;
        self.registers
            .insert(a, self.registers[&a] | self.registers[&b]);
        self.program_counter += 1;
        self.steps += 1;
        self.check_program_end();
    }

    fn xor(&mut self, a: u8, b: u8) {
        self.state = SimulationState::Running;
        self.registers
            .insert(a, self.registers[&a] ^ self.registers[&b]);
        self.program_counter += 1;
        self.steps += 1;
        self.check_program_end();
    }

    fn not(&mut self, a: u8) {
        self.state = SimulationState::Running;
        self.registers.insert(a, !self.registers[&a]);
        self.program_counter += 1;
        self.steps += 1;
        self.check_program_end();
    }

    fn set(&mut self, a: u8, value: u8) {
        self.state = SimulationState::Running;
        self.registers.insert(a, 0 != value);
        self.program_counter += 1;
        self.steps += 1;
        self.check_program_end();
    }

    fn random(&mut self, a: u8) {
        self.state = SimulationState::Running;
        self.registers.insert(a, self.rng.gen());
        self.program_counter += 1;
        self.steps += 1;
        self.check_program_end();
    }

    fn jump(&mut self, a: usize) {
        self.state = SimulationState::Running;
        self.steps += 1;
        self.program_counter = a;
        self.check_program_end();
    }

    fn jz(&mut self, a: usize, b: u8) {
        self.state = SimulationState::Running;
        self.steps += 1;
        if self.registers[&b] {
            self.program_counter += 1;
        } else {
            self.program_counter = a;
        }
        self.check_program_end();
    }

    fn halt(&mut self) {
        self.state = SimulationState::Halted;
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

    #[test]
    fn simulation_random() {
        let mut simulation = Simulation::new();
        simulation.random(0);
        assert_eq!(true, simulation.registers[&0]);
    }

    #[test]
    fn simulation_jump() {
        let mut simulation = Simulation::new();
        simulation.jump(10);
        assert_eq!(10, simulation.program_counter);
    }

    #[test]
    fn simulation_jz() {
        let mut simulation = Simulation::new();
        simulation.registers.insert(0, true);
        simulation.jz(10, 0);
        assert_eq!(1, simulation.program_counter);
        simulation.registers.insert(0, false);
        simulation.jz(10, 0);
        assert_eq!(10, simulation.program_counter);
    }

    #[test]
    fn simulation_halt() {
        let mut simulation = Simulation::new();
        simulation.halt();
        assert_eq!(SimulationState::Halted, simulation.state);
    }
}
