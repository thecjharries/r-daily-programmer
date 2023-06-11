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

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn print_song() -> String {
    todo!()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_song() {
        let output = "Hi ho, the barren moor,\nThe moor down in the valley-o,\nHi ho, the barren moor,\nThe moor down in the valley-o.\nNow in that moor there was a tree,\nA bare tree, a barren tree;\nThe tree in the moorAnd the moor down in the valley-o\nHi ho, the barren moor,\nThe moor down in the valley-o,\nHi ho, the barren moor,\nThe moor down in the valley-o.\nNow on that tree there was a branch,\nA bare branch, a barren branch;\nThe branch on the treeAnd the tree in the moorAnd the moor down in the valley-o\nHi ho, the barren moor,\nThe moor down in the valley-o,\nHi ho, the barren moor,\nThe moor down in the valley-o.\nNow on that branch there was a nest,\nA bare nest, a barren nest;\nThe nest on the branchAnd the branch on the treeAnd the tree in the moorAnd the moor down in the valley-o\nHi ho, the barren moor,\nThe moor down in the valley-o,\nHi ho, the barren moor,\nThe moor down in the valley-o.\nNow in that nest there was a bird,\nA bare bird, a barren bird;\nThe bird in the nestAnd the nest on the branchAnd the branch on the treeAnd the tree in the moorAnd the moor down in the valley-o\nHi ho, the barren moor,\nThe moor down in the valley-o,\nHi ho, the barren moor,\nThe moor down in the valley-o.\nNow under that bird there was a egg,\nA bare egg, a barren egg;\nThe egg under the birdAnd the bird in the nestAnd the nest on the branchAnd the branch on the treeAnd the tree in the moorAnd the moor down in the valley-o\nHi ho, the barren moor,\nThe moor down in the valley-o,\nHi ho, the barren moor,\nThe moor down in the valley-o.\nNow in that egg there was a chick,\nA bare chick, a barren chick;\nThe chick in the eggAnd the egg under the birdAnd the bird in the nestAnd the nest on the branchAnd the branch on the treeAnd the tree in the moorAnd the moor down in the valley-o\nHi ho, the barren moor,\nThe moor down in the valley-o,\nHi ho, the barren moor,\nThe moor down in the valley-o.\nNow in that chick there was a heart,\nA bare heart, a barren heart;\nThe heart in the chickAnd the chick in the eggAnd the egg under the birdAnd the bird in the nestAnd the nest on the branchAnd the branch on the treeAnd the tree in the moorAnd the moor down in the valley-o\nHi ho, the barren moor,\nThe moor down in the valley-o,\nHi ho, the barren moor,\nThe moor down in the valley-o.\nNow in that heart there was a love,\nA bare love, a barren love;\nThe love in the heartAnd the heart in the chickAnd the chick in the eggAnd the egg under the birdAnd the bird in the nestAnd the nest on the branchAnd the branch on the treeAnd the tree in the moorAnd the moor down in the valley-o\n".to_string();
        assert_eq!(output, print_song());
    }
}
