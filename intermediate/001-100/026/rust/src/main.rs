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

use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Employee {
    name: String,
    age: u32,
    salary: f32,
}

impl FromStr for Employee {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut parts = input.split(", ");
        if 3 != parts.clone().count() {
            return Err("Invalid input".to_string());
        }
        let name = parts.next().unwrap().to_string();
        let age = parts.next().unwrap().parse::<u32>().unwrap();
        let salary = parts
            .next()
            .unwrap()
            .replace("$", "")
            .replace(" per hour", "")
            .parse::<f32>()
            .unwrap();
        Ok(Employee { name, age, salary })
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
    fn test_employee_from_str() {
        let input = "New Years Baby, 1, $12.00 per hour";
        let expected = Employee {
            name: "New Years Baby".to_string(),
            age: 1,
            salary: 12.00,
        };
        assert_eq!(expected, Employee::from_str(input).unwrap());
        assert_eq!(
            "Invalid input",
            Employee::from_str("Invalid input").unwrap_err()
        );
    }
}
