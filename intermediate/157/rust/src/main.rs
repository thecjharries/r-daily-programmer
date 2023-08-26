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

use cubesim::{Cube, Face, FaceletCube, Move, MoveVariant};

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn parse_move(input: &str) -> Option<Move> {
    let binding = input.to_uppercase();
    let binding = binding.trim();
    match binding {
        "U" => Some(Move::U(MoveVariant::Standard)),
        "U2" => Some(Move::U(MoveVariant::Double)),
        "U'" => Some(Move::U(MoveVariant::Inverse)),
        "D" => Some(Move::D(MoveVariant::Standard)),
        "D2" => Some(Move::D(MoveVariant::Double)),
        "D'" => Some(Move::D(MoveVariant::Inverse)),
        "L" => Some(Move::L(MoveVariant::Standard)),
        "L2" => Some(Move::L(MoveVariant::Double)),
        "L'" => Some(Move::L(MoveVariant::Inverse)),
        "R" => Some(Move::R(MoveVariant::Standard)),
        "R2" => Some(Move::R(MoveVariant::Double)),
        "R'" => Some(Move::R(MoveVariant::Inverse)),
        "F" => Some(Move::F(MoveVariant::Standard)),
        "F2" => Some(Move::F(MoveVariant::Double)),
        "F'" => Some(Move::F(MoveVariant::Inverse)),
        "B" => Some(Move::B(MoveVariant::Standard)),
        "B2" => Some(Move::B(MoveVariant::Double)),
        "B'" => Some(Move::B(MoveVariant::Inverse)),
        _ => None,
    }
}

fn get_final_front_face(moves: &str) -> String {
    let moves = moves.split_whitespace();
    let mut cube = FaceletCube::new(3);
    for mv in moves {
        if let Some(parsed_move) = parse_move(mv) {
            cube = cube.apply_move(parsed_move);
        }
    }
    let mut output = String::new();
    let mut count = 0;
    for face in cube.state()[18..27].iter() {
        count += 1;
        match face {
            Face::F => output.push('r'),
            Face::R => output.push('g'),
            Face::B => output.push('o'),
            Face::L => output.push('b'),
            Face::U => output.push('y'),
            Face::D => output.push('w'),
            _ => output.push(' '),
        }
        if 0 == count % 3 {
            output.push('\n');
        }
    }
    output
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_move_handles_all_input() {
        assert_eq!(Move::U(MoveVariant::Standard), parse_move("U").unwrap());
        assert_eq!(Move::U(MoveVariant::Double), parse_move("U2").unwrap());
        assert_eq!(Move::U(MoveVariant::Inverse), parse_move("U'").unwrap());
        assert_eq!(Move::D(MoveVariant::Standard), parse_move("D").unwrap());
        assert_eq!(Move::D(MoveVariant::Double), parse_move("D2").unwrap());
        assert_eq!(Move::D(MoveVariant::Inverse), parse_move("D'").unwrap());
        assert_eq!(Move::L(MoveVariant::Standard), parse_move("L").unwrap());
        assert_eq!(Move::L(MoveVariant::Double), parse_move("L2").unwrap());
        assert_eq!(Move::L(MoveVariant::Inverse), parse_move("L'").unwrap());
        assert_eq!(Move::R(MoveVariant::Standard), parse_move("R").unwrap());
        assert_eq!(Move::R(MoveVariant::Double), parse_move("R2").unwrap());
        assert_eq!(Move::R(MoveVariant::Inverse), parse_move("R'").unwrap());
        assert_eq!(Move::F(MoveVariant::Standard), parse_move("F").unwrap());
        assert_eq!(Move::F(MoveVariant::Double), parse_move("F2").unwrap());
        assert_eq!(Move::F(MoveVariant::Inverse), parse_move("F'").unwrap());
        assert_eq!(Move::B(MoveVariant::Standard), parse_move("B").unwrap());
        assert_eq!(Move::B(MoveVariant::Double), parse_move("B2").unwrap());
        assert_eq!(Move::B(MoveVariant::Inverse), parse_move("B'").unwrap());
        assert_eq!(Move::U(MoveVariant::Standard), parse_move("u").unwrap());
        assert_eq!(Move::U(MoveVariant::Standard), parse_move(" U  ").unwrap());
        assert!(parse_move("qqq").is_none());
    }

    #[test]
    fn get_final_front_face_parse_properly() {
        let output = "rrb\nrrw\noww\n";
        assert_eq!(
            output,
            get_final_front_face("U2 R' D2 R F L' U2 R").as_str()
        );
    }
}
