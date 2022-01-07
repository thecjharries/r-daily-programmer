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

fn password_protected(username: &str, password: &str) -> bool {
    username == "AzureDiamond" && password == "hunter2"
}

fn main() {
    let username = "AzureDiamond";
    let password = "hunter2";
    println!("{}", password_protected(username, password));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_protected() {
        assert_eq!(password_protected("AzureDiamond", "hunter2"), true);
        assert_eq!(password_protected("AzureDiamond", "hunter3"), false);
        assert_eq!(password_protected("qqq", "hunter2"), false);
        assert_eq!(password_protected("aaa", "hunter22"), false);
    }
}
