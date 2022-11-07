// Copyright 2022 CJ Harries
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

fn find_sequence_path(sequence: Vec<char>, maze: Vec<Vec<char>>) -> Vec<(usize, usize)> {
    todo!()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(
            vec![(4, 1), (3, 1), (2, 1), (2, 2), (2, 3), (1, 3), (0, 3)],
            find_sequence_path(
                vec!['O', 'G'],
                vec![
                    vec!['B', 'O', 'R', 'O', 'Y'],
                    vec!['O', 'R', 'B', 'G', 'R'],
                    vec!['B', 'O', 'G', 'O', 'Y'],
                    vec!['Y', 'G', 'B', 'Y', 'G'],
                    vec!['R', 'O', 'R', 'B', 'R'],
                ]
            ),
        );
        assert_eq!(
            vec![
                (19, 6),
                (19, 5),
                (19, 4),
                (19, 3),
                (18, 3),
                (17, 3),
                (16, 3),
                (16, 4),
                (15, 4),
                (16, 4),
                (16, 5),
                (15, 5),
                (14, 5),
                (14, 6),
                (13, 6),
                (12, 6),
                (11, 6),
                (10, 6),
                (10, 7),
                (9, 7),
                (9, 8),
                (8, 8),
                (8, 9),
                (7, 9),
                (6, 9),
                (6, 10),
                (5, 10),
                (4, 10),
                (3, 10),
                (4, 10),
                (4, 9),
                (4, 8),
                (3, 8),
                (2, 8),
                (1, 8),
                (0, 8)
            ],
            find_sequence_path(
                vec!['R', 'O', 'Y', 'P', 'O'],
                vec![
                    vec![
                        'R', 'R', 'B', 'R', 'R', 'R', 'B', 'P', 'Y', 'G', 'P', 'B', 'B', 'B', 'G',
                        'P', 'B', 'P', 'P', 'R'
                    ],
                    vec![
                        'B', 'G', 'Y', 'P', 'R', 'P', 'Y', 'Y', 'O', 'R', 'Y', 'P', 'P', 'Y', 'Y',
                        'R', 'R', 'R', 'P', 'P'
                    ],
                    vec![
                        'B', 'P', 'G', 'R', 'O', 'P', 'Y', 'G', 'R', 'Y', 'Y', 'G', 'P', 'O', 'R',
                        'Y', 'P', 'B', 'O', 'O'
                    ],
                    vec![
                        'R', 'B', 'B', 'O', 'R', 'P', 'Y', 'O', 'O', 'Y', 'R', 'P', 'B', 'R', 'G',
                        'R', 'B', 'G', 'P', 'G'
                    ],
                    vec![
                        'R', 'P', 'Y', 'G', 'G', 'G', 'P', 'Y', 'P', 'Y', 'O', 'G', 'B', 'O', 'R',
                        'Y', 'P', 'B', 'Y', 'O'
                    ],
                    vec![
                        'O', 'R', 'B', 'G', 'B', 'Y', 'B', 'P', 'G', 'R', 'P', 'Y', 'R', 'O', 'G',
                        'Y', 'G', 'Y', 'R', 'P'
                    ],
                    vec![
                        'B', 'G', 'O', 'O', 'O', 'G', 'B', 'B', 'R', 'O', 'Y', 'Y', 'Y', 'Y', 'P',
                        'B', 'Y', 'Y', 'G', 'G'
                    ],
                    vec![
                        'P', 'P', 'G', 'B', 'O', 'P', 'Y', 'G', 'B', 'R', 'O', 'G', 'B', 'G', 'R',
                        'O', 'Y', 'R', 'B', 'R'
                    ],
                    vec![
                        'Y', 'Y', 'P', 'P', 'R', 'B', 'Y', 'B', 'P', 'O', 'O', 'G', 'P', 'Y', 'R',
                        'P', 'P', 'Y', 'R', 'Y'
                    ],
                    vec![
                        'P', 'O', 'O', 'B', 'B', 'B', 'G', 'O', 'Y', 'G', 'O', 'P', 'B', 'G', 'Y',
                        'R', 'R', 'Y', 'R', 'B'
                    ],
                    vec![
                        'P', 'P', 'Y', 'R', 'B', 'O', 'O', 'R', 'O', 'R', 'Y', 'B', 'G', 'B', 'G',
                        'O', 'O', 'P', 'B', 'Y'
                    ],
                    vec![
                        'B', 'B', 'R', 'G', 'Y', 'G', 'P', 'Y', 'G', 'P', 'R', 'R', 'P', 'Y', 'G',
                        'O', 'O', 'Y', 'R', 'R'
                    ],
                    vec![
                        'O', 'G', 'R', 'Y', 'B', 'P', 'Y', 'O', 'P', 'B', 'R', 'Y', 'B', 'G', 'P',
                        'G', 'O', 'O', 'B', 'P'
                    ],
                    vec![
                        'R', 'Y', 'G', 'P', 'G', 'G', 'O', 'R', 'Y', 'O', 'O', 'G', 'R', 'G', 'P',
                        'P', 'Y', 'P', 'B', 'G'
                    ],
                    vec![
                        'P', 'Y', 'P', 'R', 'O', 'O', 'R', 'O', 'Y', 'R', 'P', 'O', 'R', 'Y', 'P',
                        'Y', 'B', 'B', 'Y', 'R'
                    ],
                    vec![
                        'O', 'Y', 'P', 'G', 'R', 'P', 'R', 'G', 'P', 'O', 'B', 'B', 'R', 'B', 'O',
                        'B', 'Y', 'Y', 'B', 'P'
                    ],
                    vec![
                        'B', 'Y', 'Y', 'P', 'O', 'Y', 'O', 'Y', 'O', 'R', 'B', 'R', 'G', 'G', 'Y',
                        'G', 'R', 'G', 'Y', 'G'
                    ],
                    vec![
                        'Y', 'B', 'Y', 'Y', 'G', 'B', 'R', 'R', 'O', 'B', 'O', 'P', 'P', 'O', 'B',
                        'O', 'R', 'R', 'R', 'P'
                    ],
                    vec![
                        'P', 'O', 'O', 'O', 'P', 'Y', 'G', 'G', 'Y', 'P', 'O', 'G', 'P', 'O', 'B',
                        'G', 'P', 'R', 'P', 'B'
                    ],
                    vec![
                        'R', 'B', 'B', 'R', 'R', 'R', 'R', 'B', 'B', 'B', 'Y', 'O', 'B', 'G', 'P',
                        'G', 'G', 'O', 'O', 'Y'
                    ],
                ]
            )
        );
    }
}
