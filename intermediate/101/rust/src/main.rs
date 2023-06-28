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

#[derive(Debug, PartialEq, Eq)]
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
}
