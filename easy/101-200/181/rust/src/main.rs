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

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref SLOPE_INTERCEPT_PATTERN: Regex =
        Regex::new(r"^y=(?:(.*)x)?(?:\+?([^x]*))?$").unwrap();
}

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn find_intersection(first: &str, second: &str) -> Result<(f32, f32), String> {
    let first_slope: f32;
    let first_intercept: f32;
    let second_slope: f32;
    let second_intercept: f32;
    match SLOPE_INTERCEPT_PATTERN.captures(first) {
        Some(captures) => {
            first_slope = captures
                .get(1)
                .map(|m| m.as_str().parse::<f32>().unwrap_or(1.0))
                .unwrap_or(0.0);
            first_intercept = captures
                .get(2)
                .map(|m| m.as_str().parse::<f32>().unwrap_or(0.0))
                .unwrap_or(0.0);
        }
        None => return Err(String::from("Invalid first line")),
    }
    match SLOPE_INTERCEPT_PATTERN.captures(second) {
        Some(captures) => {
            second_slope = captures
                .get(1)
                .map(|m| m.as_str().parse::<f32>().unwrap_or(1.0))
                .unwrap_or(0.0);
            second_intercept = captures
                .get(2)
                .map(|m| m.as_str().parse::<f32>().unwrap_or(0.0))
                .unwrap_or(0.0);
        }
        None => return Err(String::from("Invalid second line")),
    }
    let x = (second_intercept - first_intercept) / (first_slope - second_slope);
    if x.is_infinite() {
        return Err(String::from("Lines are parallel"));
    }
    if x.is_nan() {
        return Err(String::from("Lines are parallel"));
    }
    let y = first_slope * x + first_intercept;
    Ok((x, y))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_intersection() {
        assert!(find_intersection("y=0", "y=0").is_err());
        assert!(find_intersection("y=x", "y=x+1").is_err());
        assert!(find_intersection("qqq", "y=x+1").is_err());
        assert!(find_intersection("y=x+1", "qqq").is_err());
        assert_eq!(Ok((2.0, 6.0)), find_intersection("y=2x+2", "y=5x-4"));
        assert_eq!(
            Ok((-0.7894737, 0.90526307)),
            find_intersection("y=0.5x+1.3", "y=-1.4x-0.2")
        );
    }
}
