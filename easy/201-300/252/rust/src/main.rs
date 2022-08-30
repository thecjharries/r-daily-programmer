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

fn determine_pool_size(sailors: u32) -> u32 {
    if 2 > sailors {
        return 0;
    } else if 2 == sailors {
        return 11;
    }
    if 0 == sailors % 2 {
        return (sailors - 1) * (sailors.pow(sailors) - 1);
    } else {
        return sailors.pow(sailors) - sailors + 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(0, determine_pool_size(1));
        assert_eq!(11, determine_pool_size(2));
        assert_eq!(3121, determine_pool_size(5));
        assert_eq!(233275, determine_pool_size(6));
    }
}
