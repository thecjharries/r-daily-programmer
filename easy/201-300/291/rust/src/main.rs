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

fn find_seats(weight: u32, temp: u32, seats: Vec<(u32, u32)>) -> Vec<usize> {
    todo!()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_seats() {
        let mut seats = vec![
            (30, 50),
            (130, 75),
            (90, 60),
            (150, 85),
            (120, 70),
            (200, 200),
            (110, 100),
        ];
        assert_eq!(vec![1, 4], find_seats(100, 80, seats));
        seats = vec![
            (297, 90),
            (66, 110),
            (257, 113),
            (276, 191),
            (280, 129),
            (219, 163),
            (254, 193),
            (86, 153),
            (206, 147),
            (71, 137),
            (104, 40),
            (238, 127),
            (52, 146),
            (129, 197),
            (144, 59),
            (157, 124),
            (210, 59),
            (11, 54),
            (268, 119),
            (261, 121),
            (12, 189),
            (186, 108),
            (174, 21),
            (77, 18),
            (54, 90),
            (174, 52),
            (16, 129),
            (59, 181),
            (290, 123),
            (248, 132),
        ];
        assert_eq!(
            vec![0, 2, 10, 14, 16, 18, 21, 22, 25],
            find_seats(100, 120, seats)
        );
    }
}
