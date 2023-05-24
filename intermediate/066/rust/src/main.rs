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

use std::collections::BTreeMap;

struct MaxiphobicMap<T>(BTreeMap<String, T>);

impl<T> MaxiphobicMap<T> {
    fn new() -> Self {
        Self(BTreeMap::new())
    }

    fn insert(&mut self, key: String, value: T) -> Option<T> {
        self.0.insert(key, value)
    }

    fn get(&self, key: &str) -> Option<&T> {
        self.0.get(key)
    }

    fn remove(&mut self, key: &str) -> Option<T> {
        self.0.remove(key)
    }

    fn merge(self, other: Self) -> Self {
        let mut new_map = self.0;
        for (key, value) in other.0 {
            new_map.insert(key, value);
        }
        Self(new_map)
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
    fn test_maxiphobicmap_basics() {
        let mut map: MaxiphobicMap<u32> = MaxiphobicMap::new();
        assert_eq!(None, map.get("foo"));
        assert_eq!(None, map.remove("foo"));
        assert_eq!(None, map.insert("foo".to_string(), 1));
        assert_eq!(Some(&1), map.get("foo"));
        assert_eq!(Some(1u32), map.remove("foo"));
        assert_eq!(None, map.get("foo"));
    }

    #[test]
    fn test_maxiphobicmap_merge() {
        let mut map: MaxiphobicMap<u32> = MaxiphobicMap::new();
        map.insert("foo".to_string(), 1);
        let mut other_map: MaxiphobicMap<u32> = MaxiphobicMap::new();
        other_map.insert("bar".to_string(), 2);
        let merged_map = map.merge(other_map);
        assert_eq!(Some(&1), merged_map.get("foo"));
        assert_eq!(Some(&2), merged_map.get("bar"));
    }
}
