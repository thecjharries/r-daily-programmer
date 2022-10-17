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

use gcd::Gcd;

#[derive(Debug, PartialEq)]
enum Corner {
    UL,
    UR,
    LR,
    LL,
}

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn determine_end(height: u32, width: u32, velocity: u32) -> (Corner, u32, u32) {
    let path = height * width / height.gcd(width);
    let time = path / velocity.gcd(path);
    let rounds = velocity * time / path;
    let mut corner: Corner = Corner::UL;
    if 1 == (velocity * time / height) % 2 {
        if 1 == (velocity * time / width) % 2 {
            corner = Corner::LR;
        } else {
            corner = Corner::LL;
        }
    } else {
        if 1 == (velocity * time / width) % 2 {
            corner = Corner::UR;
        }
    }
    let bounces = (path / height + path / width - 1) * rounds - 1;
    (corner, bounces, time)
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!((Corner::LL, 9, 24), determine_end(8, 3, 1));
        assert_eq!((Corner::UR, 17, 30), determine_end(15, 4, 2));
        assert_eq!((Corner::LR, 6, 30), determine_end(10, 6, 1));
    }
}
