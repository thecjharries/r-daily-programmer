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

fn expand_symbolic_links(path: &str, links: Vec<(String, String)>) -> String {
    let mut path = path.to_string();
    let mut found = true;
    while found {
        found = false;
        for (from, to) in links.iter() {
            if path.starts_with(from) {
                path = path.replace(from, to);
                found = true;
            }
        }
    }
    path
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand_symbolic_links() {
        let mut links = vec![
            ("/bin/thing", "/bin/thing-3"),
            ("/bin/thing-3", "/bin/thing-3.2"),
            ("/bin/thing-3.2/include", "/usr/include"),
            ("/usr/include/SDL", "/usr/local/include/SDL"),
        ];
        assert_eq!(
            "/usr/local/include/SDL/stan",
            expand_symbolic_links("/bin/thing/include/SDL/stan", links)
        );
        links = vec![
            ("/bin", "/usr/bin"),
            ("/usr/bin", "/usr/local/bin/"),
            ("/usr/local/bin/log", "/var/log-2014"),
        ];
        assert_eq!(
            "/var/log-2014/rc",
            expand_symbolic_links("/bin/log/rc", links)
        );
    }
}
