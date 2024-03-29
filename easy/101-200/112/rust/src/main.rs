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

use url::Url;

fn main() {
    println!("rad");
}

fn parse_query_params(url: &str) -> Vec<(String, String)> {
    let result: Vec<(String, String)> = Vec::new();
    let url = Url::parse(url);
    match url {
        Ok(url) => url
            .query_pairs()
            .into_owned()
            .collect::<Vec<(String, String)>>(),
        Err(_) => result,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_query_params() {
        assert_eq!(
            parse_query_params("http://en.wikipedia.org/w/index.php?title=Main_Page&action=edit"),
            vec![
                ("title".to_string(), "Main_Page".to_string()),
                ("action".to_string(), "edit".to_string())
            ]
        );
        assert_eq!(
            parse_query_params("http://en.wikipedia.org/w/index.php"),
            vec![]
        );
        assert_eq!(
            parse_query_params("http:en.wikipedia.org/w/index.php"),
            vec![]
        );
    }
}
