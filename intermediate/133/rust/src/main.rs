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

#[derive(Debug, PartialEq, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn new(input: char) -> Option<Self> {
        match input.to_ascii_uppercase() {
            'U' => Some(Direction::Up),
            'D' => Some(Direction::Down),
            'L' => Some(Direction::Left),
            'R' => Some(Direction::Right),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Element {
    letter: char,
    radius: usize,
    directions: Vec<Direction>,
    reacted: bool,
}

impl Element {
    fn new(letter: char, radius: usize, directions: &str) -> Self {
        let directions = directions
            .chars()
            .filter_map(Direction::new)
            .collect::<Vec<Direction>>();
        Self {
            letter,
            radius,
            directions,
            reacted: false,
        }
    }
}

impl std::fmt::Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.reacted {
            write!(f, "X")
        } else {
            write!(f, "{}", self.letter.to_ascii_lowercase())
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct ElementGrid {
    starting_point: (usize, usize),
    elements: Vec<Vec<Option<Element>>>,
}

impl ElementGrid {
    fn new(square_size: usize, input: Vec<(usize, usize, Element)>) -> Self {
        let mut elements = vec![vec![None; square_size]; square_size];
        for (x, y, element) in input.iter() {
            elements[*y][*x] = Some(element.clone());
        }
        Self {
            starting_point: (input[0].0, input[0].1),
            elements,
        }
    }

    fn react(&mut self) -> Vec<String> {
        let mut reactions = vec![format!("{}", self)];
        let mut available_elements = vec![(self.starting_point.0, self.starting_point.1)];
        let length = self.elements.len();
        while 0 < available_elements.len() {
            let (x, y) = available_elements.pop().unwrap();
            if let Some(element) = self.elements[y][x].as_mut() {
                if !element.reacted {
                    println!("Element: {}", element);
                    element.reacted = true;
                    for direction in element.directions.iter() {
                        let x_range = match direction {
                            Direction::Left => x.saturating_sub(element.radius)..x,
                            Direction::Right => {
                                x + 1..x.saturating_add(element.radius + 1).min(length)
                            }
                            Direction::Up | Direction::Down => x..x + 1,
                        };
                        for x in x_range {
                            available_elements.push((x, y));
                        }
                        let y_range = match direction {
                            Direction::Up => y.saturating_sub(element.radius)..y,
                            Direction::Down => {
                                y + 1..y.saturating_add(element.radius + 1).min(length)
                            }
                            Direction::Left | Direction::Right => y..y + 1,
                        };
                        for y in y_range {
                            available_elements.push((x, y));
                        }
                    }
                    reactions.push(format!("{}", self));
                }
            }
        }
        reactions
    }
}

impl std::fmt::Display for ElementGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut output = String::new();
        for row in self.elements.iter() {
            for element in row.iter() {
                if let Some(element) = element {
                    output.push_str(&format!("{}", element));
                } else {
                    output.push_str(" ");
                }
            }
            output.push_str("\n");
        }
        write!(f, "{}", output)
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
    fn direction_new_creates_all_possibilities() {
        assert_eq!(Direction::new('U'), Some(Direction::Up));
        assert_eq!(Direction::new('D'), Some(Direction::Down));
        assert_eq!(Direction::new('L'), Some(Direction::Left));
        assert_eq!(Direction::new('R'), Some(Direction::Right));
        assert_eq!(Direction::new('u'), Some(Direction::Up));
        assert_eq!(Direction::new('d'), Some(Direction::Down));
        assert_eq!(Direction::new('l'), Some(Direction::Left));
        assert_eq!(Direction::new('r'), Some(Direction::Right));
        assert_eq!(Direction::new('a'), None);
        assert_eq!(Direction::new('A'), None);
    }

    #[test]
    fn elements_can_be_created() {
        let element = Element::new('a', 1, "Ud");
        assert_eq!(
            element,
            Element {
                letter: 'a',
                radius: 1,
                directions: vec![Direction::Up, Direction::Down],
                reacted: false,
            }
        );
    }

    #[test]
    fn elements_change_display_when_reacted() {
        let element = Element {
            letter: 'a',
            radius: 1,
            directions: vec![Direction::Up],
            reacted: false,
        };
        assert_eq!(format!("{}", element), "a");
        let element = Element {
            letter: 'a',
            radius: 1,
            directions: vec![Direction::Up],
            reacted: true,
        };
        assert_eq!(format!("{}", element), "X");
    }

    #[test]
    fn elementgrids_can_be_created() {
        let grid = ElementGrid::new(
            5,
            vec![
                (0, 0, Element::new('A', 5, "udlr")),
                (4, 0, Element::new('B', 5, "ud")),
                (4, 2, Element::new('C', 2, "lr")),
                (2, 3, Element::new('D', 3, "udlr")),
            ],
        );
        assert_eq!(
            grid,
            ElementGrid {
                starting_point: (0, 0),
                elements: vec![
                    vec![
                        Some(Element::new('A', 5, "udlr")),
                        None,
                        None,
                        None,
                        Some(Element::new('B', 5, "ud")),
                    ],
                    vec![None, None, None, None, None],
                    vec![None, None, None, None, Some(Element::new('C', 2, "lr"))],
                    vec![None, None, Some(Element::new('D', 3, "udlr")), None, None],
                    vec![None, None, None, None, None],
                ],
            }
        );
    }

    #[test]
    fn elementgrids_can_be_printed() {
        let grid = ElementGrid::new(
            5,
            vec![
                (0, 0, Element::new('A', 5, "udlr")),
                (4, 0, Element::new('B', 5, "ud")),
                (4, 2, Element::new('C', 2, "lr")),
                (2, 3, Element::new('D', 3, "udlr")),
            ],
        );
        assert_eq!(
            "a   b\n     \n    c\n  d  \n     \n".to_string(),
            format!("{}", grid)
        );
    }

    #[test]
    fn elementgrid_reactions_are_traced() {
        let mut grid = ElementGrid::new(
            5,
            vec![
                (0, 0, Element::new('A', 5, "udlr")),
                (4, 0, Element::new('B', 5, "ud")),
                (4, 2, Element::new('C', 2, "lr")),
                (2, 3, Element::new('D', 3, "udlr")),
            ],
        );
        assert_eq!(
            vec![
                "a   b\n     \n    c\n  d  \n     \n",
                "X   b\n     \n    c\n  d  \n     \n",
                "X   X\n     \n    c\n  d  \n     \n",
                "X   X\n     \n    X\n  d  \n     \n"
            ],
            grid.react()
        )
    }
}
