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

use std::cmp::Ord;
use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
struct GenericSet<T> {
    items: BTreeMap<T, bool>,
}

impl<T: PartialEq + Clone + Ord> GenericSet<T> {
    fn new() -> Self {
        Self {
            items: BTreeMap::new(),
        }
    }

    fn from_slice(slice: &[T]) -> Self {
        let mut items = BTreeMap::new();
        for item in slice {
            items.insert(item.clone(), true);
        }
        Self { items }
    }

    fn add(&mut self, item: Vec<T>) {
        for item in item {
            self.items.insert(item, true);
        }
    }

    fn union(&self, other: &Self) -> Self {
        let mut items = self.items.clone();
        for (key, _) in other.items.iter() {
            items.insert(key.clone(), true);
        }
        Self { items }
    }

    fn intersection(&self, other: &Self) -> Self {
        let mut items = BTreeMap::new();
        for (key, _) in self.items.iter() {
            if other.items.contains_key(key) {
                items.insert(key.clone(), true);
            }
        }
        Self { items }
    }
}

impl<T: fmt::Debug> fmt::Display for GenericSet<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.items.keys().collect::<Vec<&T>>())
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
        assert_eq!(0, GenericSet::<u32>::new().items.len());
    }

    #[test]
    fn test_from_slice() {
        assert_eq!(
            vec![1, 2, 3],
            GenericSet::from_slice(&[1, 2, 3])
                .items
                .keys()
                .cloned()
                .collect::<Vec<u32>>()
        )
    }

    #[test]
    fn test_add() {
        let mut set = GenericSet::from_slice(&[1, 2, 3]);
        set.add(vec![4, 5, 6]);
        assert_eq!(
            vec![1, 2, 3, 4, 5, 6],
            set.items.keys().cloned().collect::<Vec<u32>>()
        );
    }

    #[test]
    fn test_display() {
        assert_eq!(
            "[1, 2, 3]",
            format!("{}", GenericSet::from_slice(&[1, 2, 3]))
        );
    }

    #[test]
    fn test_union() {
        let set_a = GenericSet::from_slice(&[1, 2, 3]);
        let set_b = GenericSet::from_slice(&[3, 4, 5]);
        assert_eq!(
            vec![1, 2, 3, 4, 5],
            set_a
                .union(&set_b)
                .items
                .keys()
                .cloned()
                .collect::<Vec<u32>>()
        );
    }

    #[test]
    fn test_intersection() {
        let set_a = GenericSet::from_slice(&[1, 2, 3]);
        let set_b = GenericSet::from_slice(&[3, 4, 5]);
        assert_eq!(
            vec![3],
            set_a
                .intersection(&set_b)
                .items
                .keys()
                .cloned()
                .collect::<Vec<u32>>()
        );
    }
}
