// Copyright 2021 CJ Harries
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

fn main() {
    // Output 'Hello World' to the console.
    println!("Hello World!");
    // Return an array of the first 100 numbers that are divisible by 3 and 5.
    let mut fizzbuzz = Vec::new();
    let mut index: i32 = 1;
    while 100 > fizzbuzz.len() {
        if 0 == index % 3 && 0 == index % 5 {
            fizzbuzz.push(index);
        }
        index += 1;
    }
    println!("{:?}", fizzbuzz);
}
