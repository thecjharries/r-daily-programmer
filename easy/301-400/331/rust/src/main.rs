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

fn execute_operation(a: f32, b: f32, operator: char) -> Result<f32, String> {
    match operator {
        '+' => Ok(a + b),
        '-' => Ok(a - b),
        '*' => Ok(a * b),
        '/' => {
            if 0.0 == b {
                Err("Cannot divide by zero".to_string())
            } else {
                Ok(a / b)
            }
        }
        '^' => {
            let mut result = a;
            for _ in 1..b as usize {
                result *= a;
            }
            Ok(result)
        }
        _ => Err(format!("Invalid operator '{}'", operator)),
    }
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_operation() {
        assert_eq!(Ok(3.0), execute_operation(1.0, 2.0, '+'));
        assert_eq!(Ok(-1.0), execute_operation(1.0, 2.0, '-'));
        assert_eq!(Ok(2.0), execute_operation(1.0, 2.0, '*'));
        assert_eq!(Ok(0.5), execute_operation(1.0, 2.0, '/'));
        assert_eq!(
            Err("Invalid operator 'a'".to_string()),
            execute_operation(1.0, 2.0, 'a')
        );
        assert_eq!(
            Err("Cannot divide by zero".to_string()),
            execute_operation(1.0, 0.0, '/')
        );
        assert_eq!(Ok(8.0), execute_operation(2.0, 3.0, '^'));
    }
}
