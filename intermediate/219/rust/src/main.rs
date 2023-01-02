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

#[derive(Debug, PartialEq, Clone)]
struct TodoList {
    items: Vec<String>,
}

impl TodoList {
    fn new() -> TodoList {
        TodoList { items: Vec::new() }
    }

    fn add(&mut self, item: String) -> &mut Self {
        self.items.push(item);
        self
    }

    fn del(&mut self, item: String) -> &mut Self {
        self.items.retain(|current_item| current_item != &item);
        self
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
    fn test_new() {
        assert_eq!(0, TodoList::new().items.len());
    }

    #[test]
    fn test_add() {
        let mut todo_list = TodoList::new();
        todo_list
            .add("test".to_string())
            .add("test2".to_string())
            .add("test3".to_string());
        assert_eq!(3, todo_list.items.len());
    }

    #[test]
    fn test_del() {
        let mut todo_list = TodoList::new();
        todo_list
            .add("test".to_string())
            .add("test2".to_string())
            .add("test3".to_string())
            .del("test2".to_string())
            .add("test4".to_string())
            .del("test5".to_string());
        assert_eq!(3, todo_list.items.len());
    }
}
