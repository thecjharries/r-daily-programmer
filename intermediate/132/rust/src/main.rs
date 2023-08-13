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

#[derive(Debug, PartialEq)]
enum TinyInput {
    Register(u8),
    Value(u8),
}

impl TinyInput {
    fn new(input: &str) -> Self {
        match input.chars().next().unwrap() {
            '[' => Self::Register(input[1..input.len() - 1].parse::<u8>().unwrap()),
            _ => Self::Value(input.parse::<u8>().unwrap()),
        }
    }
}

#[derive(Debug, PartialEq)]
#[repr(u8)]
enum TinyInstructions {
    And(TinyInput, TinyInput) = 0x00,
    Or(TinyInput, TinyInput) = 0x02,
    Xor(TinyInput, TinyInput) = 0x04,
    Not(TinyInput) = 0x06,
    Mov(TinyInput, TinyInput) = 0x07,
    Random(TinyInput) = 0x09,
    Add(TinyInput, TinyInput) = 0x0a,
    Sub(TinyInput, TinyInput) = 0x0c,
    Jump(TinyInput) = 0x0e,
    Jz(TinyInput, TinyInput) = 0x10,
    Jeq(TinyInput, TinyInput, TinyInput) = 0x14,
    Jls(TinyInput, TinyInput, TinyInput) = 0x18,
    Jgt(TinyInput, TinyInput, TinyInput) = 0x1c,
    Halt = 0xff,
    Aprint = 0x20,
    Dprint(TinyInput) = 0x21,
}

impl TinyInstructions {
    fn new(input: &str) -> Self {
        let binding = input.to_lowercase();
        let mut split = binding.split_whitespace();
        match split.next().unwrap() {
            "and" => Self::And(
                TinyInput::new(split.next().unwrap()),
                TinyInput::new(split.next().unwrap()),
            ),
            "or" => Self::Or(
                TinyInput::new(split.next().unwrap()),
                TinyInput::new(split.next().unwrap()),
            ),
            "xor" => Self::Xor(
                TinyInput::new(split.next().unwrap()),
                TinyInput::new(split.next().unwrap()),
            ),
            "not" => Self::Not(TinyInput::new(split.next().unwrap())),
            "mov" => Self::Mov(
                TinyInput::new(split.next().unwrap()),
                TinyInput::new(split.next().unwrap()),
            ),
            "random" => Self::Random(TinyInput::new(split.next().unwrap())),
            "add" => Self::Add(
                TinyInput::new(split.next().unwrap()),
                TinyInput::new(split.next().unwrap()),
            ),
            "sub" => Self::Sub(
                TinyInput::new(split.next().unwrap()),
                TinyInput::new(split.next().unwrap()),
            ),
            "jump" => Self::Jump(TinyInput::new(split.next().unwrap())),
            "jz" => Self::Jz(
                TinyInput::new(split.next().unwrap()),
                TinyInput::new(split.next().unwrap()),
            ),
            "jeq" => Self::Jeq(
                TinyInput::new(split.next().unwrap()),
                TinyInput::new(split.next().unwrap()),
                TinyInput::new(split.next().unwrap()),
            ),
            "jls" => Self::Jls(
                TinyInput::new(split.next().unwrap()),
                TinyInput::new(split.next().unwrap()),
                TinyInput::new(split.next().unwrap()),
            ),
            "jgt" => Self::Jgt(
                TinyInput::new(split.next().unwrap()),
                TinyInput::new(split.next().unwrap()),
                TinyInput::new(split.next().unwrap()),
            ),
            "halt" => Self::Halt,
            "aprint" => Self::Aprint,
            "dprint" => Self::Dprint(TinyInput::new(split.next().unwrap())),
            _ => panic!("Invalid instruction"),
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
    fn tinyinput_creates_registers() {
        assert_eq!(TinyInput::Register(0), TinyInput::new("[0]"));
        assert_eq!(TinyInput::Register(1), TinyInput::new("[1]"));
        assert_eq!(TinyInput::Register(2), TinyInput::new("[2]"));
        assert_eq!(TinyInput::Register(3), TinyInput::new("[3]"));
        assert_eq!(TinyInput::Register(4), TinyInput::new("[4]"));
        assert_eq!(TinyInput::Register(5), TinyInput::new("[5]"));
        assert_eq!(TinyInput::Register(6), TinyInput::new("[6]"));
        assert_eq!(TinyInput::Register(70), TinyInput::new("[70]"));
    }

    #[test]
    fn tinyinput_creates_values() {
        assert_eq!(TinyInput::Value(0), TinyInput::new("0"));
        assert_eq!(TinyInput::Value(1), TinyInput::new("1"));
        assert_eq!(TinyInput::Value(2), TinyInput::new("2"));
        assert_eq!(TinyInput::Value(3), TinyInput::new("3"));
        assert_eq!(TinyInput::Value(4), TinyInput::new("4"));
        assert_eq!(TinyInput::Value(5), TinyInput::new("5"));
        assert_eq!(TinyInput::Value(6), TinyInput::new("6"));
        assert_eq!(TinyInput::Value(70), TinyInput::new("70"));
    }

    #[test]
    fn tinyinstructions_parses_all_instructions() {
        assert_eq!(
            TinyInstructions::And(TinyInput::Register(0), TinyInput::Value(1)),
            TinyInstructions::new("and [0] 1")
        );
        assert_eq!(
            TinyInstructions::Or(TinyInput::Register(0), TinyInput::Value(1)),
            TinyInstructions::new("or [0] 1")
        );
        assert_eq!(
            TinyInstructions::Xor(TinyInput::Register(0), TinyInput::Value(1)),
            TinyInstructions::new("xor [0] 1")
        );
        assert_eq!(
            TinyInstructions::Not(TinyInput::Register(0)),
            TinyInstructions::new("not [0]")
        );
        assert_eq!(
            TinyInstructions::Mov(TinyInput::Register(0), TinyInput::Value(1)),
            TinyInstructions::new("mov [0] 1")
        );
        assert_eq!(
            TinyInstructions::Random(TinyInput::Register(0)),
            TinyInstructions::new("random [0]")
        );
        assert_eq!(
            TinyInstructions::Add(TinyInput::Register(0), TinyInput::Value(1)),
            TinyInstructions::new("add [0] 1")
        );
        assert_eq!(
            TinyInstructions::Sub(TinyInput::Register(0), TinyInput::Value(1)),
            TinyInstructions::new("sub [0] 1")
        );
        assert_eq!(
            TinyInstructions::Jump(TinyInput::Register(0)),
            TinyInstructions::new("jump [0]")
        );
        assert_eq!(
            TinyInstructions::Jz(TinyInput::Register(0), TinyInput::Value(1)),
            TinyInstructions::new("jz [0] 1")
        );
        assert_eq!(
            TinyInstructions::Jeq(
                TinyInput::Register(0),
                TinyInput::Value(1),
                TinyInput::Value(2)
            ),
            TinyInstructions::new("jeq [0] 1 2")
        );
        assert_eq!(
            TinyInstructions::Jls(
                TinyInput::Register(0),
                TinyInput::Value(1),
                TinyInput::Value(2)
            ),
            TinyInstructions::new("jls [0] 1 2")
        );
        assert_eq!(
            TinyInstructions::Jgt(
                TinyInput::Register(0),
                TinyInput::Value(1),
                TinyInput::Value(2)
            ),
            TinyInstructions::new("jgt [0] 1 2")
        );
        assert_eq!(TinyInstructions::Halt, TinyInstructions::new("halt"));
        assert_eq!(TinyInstructions::Aprint, TinyInstructions::new("aprint"));
        assert_eq!(
            TinyInstructions::Dprint(TinyInput::Register(0)),
            TinyInstructions::new("dprint [0]")
        );
    }

    #[test]
    #[should_panic]
    fn tinyinstructions_errors_with_unknown_instruction() {
        TinyInstructions::new("unknown");
    }
}
