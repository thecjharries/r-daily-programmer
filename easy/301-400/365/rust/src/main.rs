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

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn up_arrow(a: f32, b: f32, count: usize) -> f32 {
    if 1 == count {
        return a.powf(b);
    }
    if 0.0 == b {
        return 1.0;
    }
    up_arrow(a, up_arrow(a, b - 1.0, count), count - 1)
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(up_arrow(2.0, 4.0, 1), 16.0);
        assert_eq!(up_arrow(2.0, 4.0, 2), 65536.0);
    }
}
