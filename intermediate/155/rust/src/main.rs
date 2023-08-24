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

use chess_notation_parser::{Flag, Turn};

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn count_captured_pieces(moves: Vec<&str>) -> (u32, u32) {
    let mut white_score: u32 = 0;
    let mut black_score: u32 = 0;
    for row in moves {
        let turn_moves = row.split_whitespace().collect::<Vec<&str>>();
        let mut white = true;
        for turn_move in turn_moves {
            let turn = Turn::try_from(turn_move).unwrap();
            println!("{:?}", turn);
            if turn.is_capture() {
                if white {
                    white_score += 1;
                } else {
                    black_score += 1;
                }
            }
            white = !white;
        }
    }
    (white_score, black_score)
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_captured_pieces_can_score_games() {
        let input = vec![
            "e4 e5",
            "Nf3 Nc6",
            "Bb5 Nf6",
            "d3 Bc5",
            "Bxc6 dxc6",
            "h3 Nd7",
            "Be3 Bd6",
            "Nbd2 O-O",
            "O-O Re8",
            "Nc4 Nf8",
            "d4 exd4",
            "Qxd4 c5",
            "Qd3 b6",
            "Nxd6 Qxd6",
            "Qxd6 cxd6",
            "Rfd1 Bb7",
            "Rxd6 Bxe4",
            "Ne1 Rad8",
            "Rad1 Ne6",
            "Rxd8 Rxd8",
            "Rxd8+ Nxd8",
            "f3 Bd5",
            "a3 Nc6",
            "Kf2 f6",
            "Nd3 Kf8",
            "Ke2 Ke7",
            "Kd2 Kd7",
            "Nf4 Bf7",
            "b3 Ne7",
            "h4 Nd5",
        ];
        let output = (7, 7);
        assert_eq!(output, count_captured_pieces(input));
    }
}
