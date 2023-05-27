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

use brainfuck::program::Program;
use brainfuck::tape::ArrayTape;
use brainfuck::Interpreter;

#[cfg(not(tarpaulin_include))]
fn main() {
    println!(
        "{}",
        parse_prompt(
            "++++++++++[>>++++++>+++++++++++>++++++++++>+++++++++>+++>+++++>++++>++++++++>+[<
    ]<-]>>+++++++.>+.-.>+++.<++++.>>+++++++.<<++.+.>+++++.>.<<-.>---.<-----.-.+++++.
    >>>+++.-.<<-.<+..----.>>>>++++++++.>+++++++..<<<<+.>>>>-.<<<<.++++.------.<+++++
    .---.>>>>>.<<<++.<<---.>++++++.>>>>+.<<<-.--------.<<+.>>>>>>+++.---.<-.<<<<---.
    <.>---.>>>>>>."
        )
    );
}

fn parse_prompt(input: &str) -> String {
    let mut program_input = "".as_bytes();
    let mut program_output = Vec::new();
    let program = Program::parse(input).unwrap();
    let mut interpreter =
        Interpreter::<ArrayTape>::new(program, &mut program_input, &mut program_output);
    interpreter.run().unwrap();
    String::from_utf8(program_output).unwrap()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(
            "Hello, World!",
            parse_prompt(
                ">++++++++[<+++++++++>-]<.>++++[<+++++++>-]<+.+++++++..+++.>>++++++[<+++++++>-]<+
        +.------------.>++++++[<+++++++++>-]<+.<.+++.------.--------.>>>++++[<++++++++>-
        ]<+."
            )
        );
    }
}
