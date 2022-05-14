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

use std::collections::HashMap;

fn main() {
    println!("rad");
}

fn compute_grades(grades: HashMap<String, Vec<f32>>) -> (f32, Vec<(String, f32)>) {
    let mut sum = 0.0;
    let mut count = 0;
    let mut result = Vec::new();
    for (name, student_grades) in &grades {
        let mut grade = student_grades.iter().sum::<f32>();
        sum += grade;
        count += student_grades.len();
        grade /= student_grades.len() as f32;
        result.push((name.to_string(), (grade * 100.0).round() / 100.0));
    }
    (((sum / count as f32) * 100.0).round() / 100.0, result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_grades() {
        let mut input = HashMap::from_iter(vec![
            ("JON".to_string(), vec![19.0, 14.0, 15.0, 15.0, 16.0]),
            ("JEREMY".to_string(), vec![15.0, 11.0, 10.0, 15.0, 16.0]),
            ("JESSE".to_string(), vec![19.0, 17.0, 20.0, 19.0, 18.0]),
        ]);
        let (avg, mut grades) = compute_grades(input);
        assert_eq!(avg, 15.93);
        grades.sort_by(|a, b| a.0.cmp(&b.0));
        assert_eq!(
            grades,
            vec![
                ("JEREMY".to_string(), 13.40),
                ("JESSE".to_string(), 18.60),
                ("JON".to_string(), 15.80),
            ]
        );
        input = HashMap::from_iter(vec![
            (
                "ABIGAIL".to_string(),
                vec![11.0, 3.0, 5.0, 20.0, 4.0, 2.0, 8.0, 17.0, 4.0, 5.0],
            ),
            (
                "ALEXANDER".to_string(),
                vec![2.0, 12.0, 20.0, 0.0, 6.0, 10.0, 3.0, 4.0, 9.0, 7.0],
            ),
            (
                "AVA".to_string(),
                vec![11.0, 15.0, 2.0, 19.0, 14.0, 5.0, 16.0, 18.0, 15.0, 19.0],
            ),
            (
                "ETHAN".to_string(),
                vec![6.0, 12.0, 0.0, 0.0, 5.0, 11.0, 0.0, 11.0, 12.0, 15.0],
            ),
            (
                "ISABELLA".to_string(),
                vec![16.0, 0.0, 10.0, 7.0, 20.0, 20.0, 7.0, 2.0, 0.0, 1.0],
            ),
            (
                "JACOB".to_string(),
                vec![2.0, 14.0, 17.0, 7.0, 1.0, 11.0, 16.0, 14.0, 14.0, 7.0],
            ),
            (
                "JAYDEN".to_string(),
                vec![10.0, 10.0, 3.0, 16.0, 15.0, 16.0, 8.0, 17.0, 15.0, 3.0],
            ),
            (
                "MADISON".to_string(),
                vec![10.0, 11.0, 19.0, 4.0, 12.0, 15.0, 7.0, 4.0, 18.0, 13.0],
            ),
            (
                "SOPHIA".to_string(),
                vec![5.0, 17.0, 14.0, 7.0, 1.0, 17.0, 18.0, 8.0, 1.0, 2.0],
            ),
            (
                "WILLIAM".to_string(),
                vec![12.0, 12.0, 19.0, 9.0, 4.0, 3.0, 0.0, 4.0, 13.0, 14.0],
            ),
        ]);
        let (avg, mut grades) = compute_grades(input);
        assert_eq!(avg, 9.50);
        grades.sort_by(|a, b| a.0.cmp(&b.0));
        assert_eq!(
            grades,
            vec![
                ("ABIGAIL".to_string(), 7.9),
                ("ALEXANDER".to_string(), 7.3),
                ("AVA".to_string(), 13.4),
                ("ETHAN".to_string(), 7.2),
                ("ISABELLA".to_string(), 8.3),
                ("JACOB".to_string(), 10.3),
                ("JAYDEN".to_string(), 11.3),
                ("MADISON".to_string(), 11.3),
                ("SOPHIA".to_string(), 9.0),
                ("WILLIAM".to_string(), 9.0)
            ]
        );
    }
}
