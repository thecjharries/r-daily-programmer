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

impl FromStr for TaskList {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tasks = s
            .split("\n")
            .map(|task| Task::from_str(task).unwrap())
            .collect();
        Ok(Self { tasks })
    }
}

impl TaskList {
    fn order(&self) -> Vec<String> {
        todo!()
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

    #[test]
    fn test_tasklist_from_str() {
        // eat_dinner: make_dinner set_table
        // make_dinner: get_milk get_meat get_veggies
        // get_meat: buy_food
        // buy_food: get_money
        // get_veggies: buy_food
        // get_money: deposit_paycheck
        let task_list = TaskList {
            tasks: vec![
                Task {
                    name: "eat_dinner".to_string(),
                    dependencies: vec!["make_dinner".to_string(), "set_table".to_string()],
                },
                Task {
                    name: "make_dinner".to_string(),
                    dependencies: vec![
                        "get_milk".to_string(),
                        "get_meat".to_string(),
                        "get_veggies".to_string(),
                    ],
                },
                Task {
                    name: "get_meat".to_string(),
                    dependencies: vec!["buy_food".to_string()],
                },
                Task {
                    name: "buy_food".to_string(),
                    dependencies: vec!["get_money".to_string()],
                },
                Task {
                    name: "get_veggies".to_string(),
                    dependencies: vec!["buy_food".to_string()],
                },
                Task {
                    name: "get_money".to_string(),
                    dependencies: vec!["deposit_paycheck".to_string()],
                },
            ],
        };
        assert_eq!(
            task_list,
            TaskList::from_str(
                "eat_dinner: make_dinner set_table\n\
                 make_dinner: get_milk get_meat get_veggies\n\
                 get_meat: buy_food\n\
                 buy_food: get_money\n\
                 get_veggies: buy_food\n\
                 get_money: deposit_paycheck"
            )
            .unwrap()
        );
    }
}
