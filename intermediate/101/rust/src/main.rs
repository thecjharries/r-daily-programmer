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

use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Rgb {
    red: u8,
    green: u8,
    blue: u8,
}

impl Default for Rgb {
    fn default() -> Self {
        Rgb {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
}

impl fmt::Display for Rgb {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, " {: >3} {: >3} {: >3}", self.red, self.green, self.blue)
    }
}

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn text_to_pbm(input: &str) -> String {
    let mut output = "P3\n".to_string();
    let mut input_bytes = input.as_bytes().to_vec();
    while 0 != input_bytes.len() % 3 {
        input_bytes.push(b' ');
    }
    let size = (input_bytes.len() as f32 / 3.0).sqrt().ceil() as usize;
    output.push_str(&format!("{} {}\n", size, size));
    let mut pixels = vec![Rgb::default(); size * size];
    for (index, byte) in input_bytes.iter().enumerate() {
        let pixel_index = index / 3;
        match index % 3 {
            0 => pixels[pixel_index].red = *byte,
            1 => pixels[pixel_index].green = *byte,
            2 => pixels[pixel_index].blue = *byte,
            _ => unreachable!(),
        }
    }
    for pixel in pixels {
        output.push_str(&format!("{}\n", pixel));
    }
    output
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb_default() {
        assert_eq!(
            Rgb::default(),
            Rgb {
                red: 0,
                green: 0,
                blue: 0
            }
        );
    }

    #[test]
    fn test_rgb_display() {
        assert_eq!("   0   0   0".to_string(), format!("{}", Rgb::default()));
        assert_eq!(
            " 255 255 255".to_string(),
            format!(
                "{}",
                Rgb {
                    red: 255,
                    green: 255,
                    blue: 255
                }
            )
        );
    }

    #[test]
    fn test_text_to_pbm() {
        assert_eq!(
            "P3\n2 2\n  97  98  99\n 100  32  32\n   0   0   0\n   0   0   0\n".to_string(),
            text_to_pbm("abcd")
        );
    }
}
