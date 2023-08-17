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

#[derive(Debug, PartialEq)]
struct FoodItem {
    name: String,
    potential_index: usize,
    after: Vec<String>,
}

impl FoodItem {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            potential_index: 0,
            after: Vec::new(),
        }
    }

    fn add_after(&mut self, after: &str) {
        self.potential_index += 1;
        self.after.push(after.to_string());
    }
}

fn build_banquet(items: Vec<&str>, relationships: Vec<(&str, &str)>) -> Vec<String> {
    todo!()
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
    fn fooditems_are_created_correctly() {
        let food_item = FoodItem::new("foo");
        assert_eq!(
            FoodItem {
                name: "foo".to_string(),
                potential_index: 0,
                after: Vec::new(),
            },
            food_item
        );
    }

    #[test]
    fn fooditems_follow_others() {
        let mut food_item = FoodItem::new("foo");
        food_item.add_after("bar");
        assert_eq!(
            FoodItem {
                name: "foo".to_string(),
                potential_index: 1,
                after: vec!["bar".to_string()],
            },
            food_item
        );
    }

    #[test]
    fn builds_banquet_correctly() {
        let items = vec!["salad", "turkey", "dessert"];
        let relationships = vec![
            ("salad", "dessert"),
            ("turkey", "dessert"),
            ("salad", "turkey"),
        ];
        let expected = vec!["salad", "turkey", "dessert"];
        assert_eq!(expected, build_banquet(items, relationships));
    }
}
