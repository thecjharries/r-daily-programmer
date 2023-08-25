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

fn build_work_list(
    work_items: Vec<(&str, usize, Option<&str>)>,
) -> (Vec<(usize, String, usize)>, usize) {
    todo!()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_work_list_naively_loads_many_workers() {
        let input = vec![
            ("Lights", 2, Some("Wiring")),
            ("Windows", 3, None),
            ("Insulation", 4, None),
            ("Painting", 4, None),
            ("Wiring", 6, None),
            ("Cleaning", 7, Some("Painting")),
        ];
        let expected = vec![
            (1, "Painting".to_string(), 4),
            (1, "Cleaning".to_string(), 7),
            (2, "Insulation".to_string(), 4),
            (3, "Wiring".to_string(), 6),
            (3, "Lights".to_string(), 2),
            (4, "Windows".to_string(), 3),
        ];
        let idle = 18;
        assert_eq!((expected, idle), build_work_list(input));
        let input = vec![
            ("Preparation", 2, Some("Planning")),
            ("Hiring", 3, None),
            ("Legal", 3, None),
            ("Briefing", 4, Some("Preparation")),
            ("Advertising", 4, None),
            ("Paperwork", 5, Some("Legal")),
            ("Testing", 5, Some("Frontend")),
            ("API", 6, None),
            ("Backend", 6, None),
            ("Planning", 7, None),
            ("Frontend", 8, None),
            ("Mobile", 8, None),
            ("Documentation", 9, Some("API")),
        ];
        let expected = vec![
            (1, "Advertising".to_string(), 4),
            (2, "Backend".to_string(), 6),
            (3, "Planning".to_string(), 7),
            (3, "Preparation".to_string(), 2),
            (3, "Briefing".to_string(), 4),
            (4, "API".to_string(), 6),
            (4, "Documentation".to_string(), 9),
            (5, "Hiring".to_string(), 3),
            (6, "Mobile".to_string(), 8),
            (7, "Legal".to_string(), 3),
            (7, "Paperwork".to_string(), 5),
            (8, "Frontend".to_string(), 8),
            (8, "Testing".to_string(), 5),
        ];
        let idle = 50;
        assert_eq!((expected, idle), build_work_list(input));
    }
}
