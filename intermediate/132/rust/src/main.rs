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
}
