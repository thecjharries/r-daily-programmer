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

struct ToDoList {
    todos: Vec<String>,
}

impl ToDoList {
    fn new() -> Self {
        Self { todos: Vec::new() }
    }

    fn add(&mut self, todo: String) {
        self.todos.push(todo);
    }
}

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_todolist_new() {
        let todos = ToDoList::new();
        assert_eq!(todos.todos.len(), 0);
    }
}
