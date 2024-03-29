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

struct SmartStack {
    stack: Vec<i32>,
    ordered: Vec<i32>,
}

impl SmartStack {
    fn new() -> SmartStack {
        SmartStack {
            stack: Vec::new(),
            ordered: Vec::new(),
        }
    }

    fn push(&mut self, value: i32) {
        self.stack.push(value);
        self.ordered.push(value);
        self.ordered.sort();
    }

    fn pop(&mut self) -> i32 {
        let value = self.stack.pop().unwrap();
        self.ordered.retain(|&x| x != value);
        self.ordered.sort();
        value
    }

    fn remove_greater_than(&mut self, value: i32) {
        self.stack.retain(|&x| x <= value);
        self.ordered.retain(|&x| x <= value);
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
    fn test_smartstack_new() {
        let stack = SmartStack::new();
        assert_eq!(stack.stack.len(), 0);
        assert_eq!(stack.ordered.len(), 0);
    }

    #[test]
    fn test_smartstack_push() {
        let mut stack = SmartStack {
            stack: vec![3, 2, 1],
            ordered: vec![1, 2, 3],
        };
        stack.push(4);
        assert_eq!(vec![3, 2, 1, 4], stack.stack);
        assert_eq!(vec![1, 2, 3, 4], stack.ordered);
    }

    #[test]
    fn test_smartstack_pop() {
        let mut stack = SmartStack {
            stack: vec![3, 2, 1],
            ordered: vec![1, 2, 3],
        };
        assert_eq!(1, stack.pop());
        assert_eq!(vec![3, 2], stack.stack);
        assert_eq!(vec![2, 3], stack.ordered);
        assert_eq!(2, stack.pop());
        assert_eq!(vec![3], stack.stack);
        assert_eq!(vec![3], stack.ordered);
        assert_eq!(3, stack.pop());
        assert_eq!(vec![] as Vec<i32>, stack.stack);
        assert_eq!(vec![] as Vec<i32>, stack.ordered);
    }

    #[test]
    fn test_smartstack_remove_greater_than() {
        let mut stack = SmartStack {
            stack: vec![3, 2, 1],
            ordered: vec![1, 2, 3],
        };
        stack.remove_greater_than(2);
        assert_eq!(vec![2, 1], stack.stack);
        assert_eq!(vec![1, 2], stack.ordered);
    }
}
