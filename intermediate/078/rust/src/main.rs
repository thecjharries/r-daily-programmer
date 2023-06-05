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
struct Task {
    name: String,
    dependencies: Vec<String>,
}

impl FromStr for Task {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let mut split = s.split(": ");
        let name = split.next().unwrap().to_string();
        let dependencies = split
            .next()
            .unwrap()
            .split(" ")
            .map(|dependency| dependency.to_string())
            .collect();
        Ok(Self { name, dependencies })
    }
}

#[derive(Debug, PartialEq)]
struct TaskList {
    tasks: Vec<Task>,
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
    fn test_task_from_str() {
        // eat_dinner: make_dinner set_table
        let task = Task {
            name: "eat_dinner".to_string(),
            dependencies: vec!["make_dinner".to_string(), "set_table".to_string()],
        };
        assert_eq!(
            task,
            Task::from_str("eat_dinner: make_dinner set_table").unwrap()
        );
    }
}
