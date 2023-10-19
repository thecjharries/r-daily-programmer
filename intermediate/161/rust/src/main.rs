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

#[derive(Debug, PartialEq, Clone)]
struct Worker {
    name: String,
    skills: Vec<String>,
}

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn build_job_list(workers: &[Worker], jobs: &[String]) -> Vec<(String, String)> {
    let mut workers = workers.to_vec();
    let mut result = Vec::new();
    for job in jobs {
        let mut worker_index = 0;
        let mut worker = &workers[worker_index];
        while !worker.skills.contains(job) && worker_index < workers.len() - 1 {
            worker_index += 1;
            worker = &workers[worker_index];
        }
        result.push((worker.name.clone(), job.clone()));
        workers.remove(worker_index);
    }
    result
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_job_list() {
        // Alice Wiring,Insulation,Plumbing
        // Bob Wiring,Decoration
        // Charlie Wiring,Plumbing
        // David Plumbing
        // Erin Insulation,Decoration,Finances
        let workers = vec![
            Worker {
                name: "Alice".to_string(),
                skills: vec![
                    "Wiring".to_string(),
                    "Insulation".to_string(),
                    "Plumbing".to_string(),
                ],
            },
            Worker {
                name: "Bob".to_string(),
                skills: vec!["Wiring".to_string(), "Decoration".to_string()],
            },
            Worker {
                name: "Charlie".to_string(),
                skills: vec!["Wiring".to_string(), "Plumbing".to_string()],
            },
            Worker {
                name: "David".to_string(),
                skills: vec!["Plumbing".to_string()],
            },
            Worker {
                name: "Erin".to_string(),
                skills: vec![
                    "Insulation".to_string(),
                    "Decoration".to_string(),
                    "Finances".to_string(),
                ],
            },
        ];
        // Wiring
        // Insulation
        // Plumbing
        // Decoration
        // Finances
        let jobs = vec![
            "Wiring".to_string(),
            "Insulation".to_string(),
            "Plumbing".to_string(),
            "Decoration".to_string(),
            "Finances".to_string(),
        ];
        // [("Alice", "Wiring"), ("Erin", "Insulation"), ("Charlie", "Plumbing"), ("Bob", "Decoration"), ("David", "Finances")]
        let expected = vec![
            ("Alice".to_string(), "Wiring".to_string()),
            ("Erin".to_string(), "Insulation".to_string()),
            ("Charlie".to_string(), "Plumbing".to_string()),
            ("Bob".to_string(), "Decoration".to_string()),
            ("David".to_string(), "Finances".to_string()),
        ];
        assert_eq!(expected, build_job_list(&workers, &jobs));
        // Alice GUI,Backend,Support
        // Bill Finances,Backend
        // Cath Documentation,Finances
        // Jack Documentation,Frontend,Support
        // Michael Frontend
        // Steve Documentation,Backend
        let workers = vec![
            Worker {
                name: "Alice".to_string(),
                skills: vec![
                    "GUI".to_string(),
                    "Backend".to_string(),
                    "Support".to_string(),
                ],
            },
            Worker {
                name: "Bill".to_string(),
                skills: vec!["Finances".to_string(), "Backend".to_string()],
            },
            Worker {
                name: "Cath".to_string(),
                skills: vec!["Documentation".to_string(), "Finances".to_string()],
            },
            Worker {
                name: "Jack".to_string(),
                skills: vec![
                    "Documentation".to_string(),
                    "Frontend".to_string(),
                    "Support".to_string(),
                ],
            },
            Worker {
                name: "Michael".to_string(),
                skills: vec!["Frontend".to_string()],
            },
            Worker {
                name: "Steve".to_string(),
                skills: vec!["Documentation".to_string(), "Backend".to_string()],
            },
        ];
        // GUI
        // Documentation
        // Finances
        // Frontend
        // Backend
        // Support
        let jobs = vec![
            "GUI".to_string(),
            "Documentation".to_string(),
            "Finances".to_string(),
            "Frontend".to_string(),
            "Backend".to_string(),
            "Support".to_string(),
        ];
        // [("Alice", "GUI"), ("Cath", "Documentation"), ("Bill", "Finances"), ("Jack", "Frontend"), ("Steve", "Backend"), ("Michael", "Support")]
        let expected = vec![
            ("Alice".to_string(), "GUI".to_string()),
            ("Cath".to_string(), "Documentation".to_string()),
            ("Bill".to_string(), "Finances".to_string()),
            ("Jack".to_string(), "Frontend".to_string()),
            ("Steve".to_string(), "Backend".to_string()),
            ("Michael".to_string(), "Support".to_string()),
        ];
        assert_eq!(expected, build_job_list(&workers, &jobs));
    }
}
