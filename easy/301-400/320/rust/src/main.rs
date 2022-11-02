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

fn build_number_spiral(size: usize) -> Vec<Vec<u32>> {
    let mut spiral = vec![vec![0; size]; size];
    let mut x: i32 = 0;
    let mut dx: i32 = 1;
    let mut y: i32 = 0;
    let mut dy: i32 = 0;
    for number in 1..=size * size {
        spiral[y as usize][x as usize] = number as u32;
        if spiral[(y - dy + size as i32) as usize % size][(x + dx + size as i32) as usize % size]
            != 0
        {
            let temp = dy;
            dy = -dx;
            dx = temp;
        }
        x += dx;
        y -= dy;
    }
    spiral
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(vec![vec![1]], build_number_spiral(1));
        assert_eq!(
            vec![
                vec![1, 2, 3, 4, 5],
                vec![16, 17, 18, 19, 6],
                vec![15, 24, 25, 20, 7],
                vec![14, 23, 22, 21, 8],
                vec![13, 12, 11, 10, 9],
            ],
            build_number_spiral(5)
        );
        assert_eq!(
            vec![
                vec![1, 2, 3, 4],
                vec![12, 13, 14, 5],
                vec![11, 16, 15, 6],
                vec![10, 9, 8, 7],
            ],
            build_number_spiral(4)
        );
    }
}
