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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_right_triangle_up_to() {
        assert_eq!(build_right_triangle_up_to(1), vec![vec![1]]);
        assert_eq!(build_right_triangle_up_to(2), vec![vec![1]]);
        assert_eq!(build_right_triangle_up_to(3), vec![vec![1], vec![2,3]]);
        assert_eq!(build_right_triangle_up_to(4), vec![vec![1], vec![2,3]]);
        assert_eq!(build_right_triangle_up_to(5), vec![vec![1], vec![2,3]]);
        assert_eq!(build_right_triangle_up_to(6), vec![vec![1], vec![2,3]], vec![4,5,6]);
    }
}
