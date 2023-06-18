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

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn generate_brainfuck_to_print(input: &str) -> String {
    let mut output = String::new();
    for character in input.chars() {
        output.push_str(&format!("{}.[-]", "+".repeat(character as u8 as usize)));
    }
    output
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;
    use brainfuck::program::Program;
    use brainfuck::tape::ArrayTape;
    use brainfuck::Interpreter;
    use std::io;
    use std::str;

    #[test]
    fn test_generate_brainfuck_to_print() {
        let input = "Hello World!";
        let mut stdin = io::stdin();
        let mut result = Vec::new();
        let program = Program::parse(&generate_brainfuck_to_print(input)).unwrap();
        let mut interp = Interpreter::<ArrayTape>::new(program, &mut stdin, &mut result);
        interp.run().unwrap();
        assert_eq!(str::from_utf8(result.as_slice()).unwrap(), input);
    }
}
