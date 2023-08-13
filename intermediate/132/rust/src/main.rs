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
    fn test_stub() {
        assert_eq!(2 + 2, 4);
    }
}
