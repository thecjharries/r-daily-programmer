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

fn determine_hamming_distance(first: &str, second: &str) -> u32 {
    first
        .chars()
        .zip(second.chars())
        .filter(|(first_char, second_char)| first_char != second_char)
        .count() as u32
}

fn find_center_word(words: Vec<&str>) -> String {
    let mut minimum_distance = u32::MAX;
    let mut center_word = String::new();
    for (index, word) in words.iter().enumerate() {
        let mut distance = 0;
        for (other_index, other_word) in words.iter().enumerate() {
            if index == other_index {
                continue;
            }
            distance += determine_hamming_distance(word, other_word);
        }
        if distance < minimum_distance {
            minimum_distance = distance;
            center_word = word.to_string();
        }
    }
    center_word
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_determine_hamming_distance() {
        assert_eq!(8, determine_hamming_distance("CTCCATCACAC", "AATATCTACAT"))
    }

    #[test]
    fn test_find_center_word() {
        assert_eq!(
            "ATTAAATAACT",
            find_center_word(vec![
                "ATCAATATCAA",
                "ATTAAATAACT",
                "AATCCTTAAAC",
                "CTACTTTCTTT",
                "TCCCATCCTTT",
                "ACTTCAATATA",
            ])
        );
        assert_eq!(
            "AATATCTACAT",
            find_center_word(vec![
                "CTCCATCACAC",
                "AATATCTACAT",
                "ACATTCTCCAT",
                "CCTCCCCACTC",
            ])
        );
        assert_eq!(
            "ATTCTACAACT",
            find_center_word(vec![
                "AACACCCTATA",
                "CTTCATCCACA",
                "TTTCAATTTTC",
                "ACAATCAAACC",
                "ATTCTACAACT",
                "ATTCCTTATTC",
                "ACTTCTCTATT",
                "TAAAACTCACC",
                "CTTTTCCCACC",
                "ACCTTTTCTCA",
                "TACCACTACTT",
            ])
        );
        assert_eq!(
            "TTAACTCCCATTATATATTATTAATTTACCC",
            find_center_word(vec![
                "ACAAAATCCTATCAAAAACTACCATACCAAT",
                "ACTATACTTCTAATATCATTCATTACACTTT",
                "TTAACTCCCATTATATATTATTAATTTACCC",
                "CCAACATACTAAACTTATTTTTTAACTACCA",
                "TTCTAAACATTACTCCTACACCTACATACCT",
                "ATCATCAATTACCTAATAATTCCCAATTTAT",
                "TCCCTAATCATACCATTTTACACTCAAAAAC",
                "AATTCAAACTTTACACACCCCTCTCATCATC",
                "CTCCATCTTATCATATAATAAACCAAATTTA",
                "AAAAATCCATCATTTTTTAATTCCATTCCTT",
                "CCACTCCAAACACAAAATTATTACAATAACA",
                "ATATTTACTCACACAAACAATTACCATCACA",
                "TTCAAATACAAATCTCAAAATCACCTTATTT",
                "TCCTTTAACAACTTCCCTTATCTATCTATTC",
                "CATCCATCCCAAAACTCTCACACATAACAAC",
                "ATTACTTATACAAAATAACTACTCCCCAATA",
                "TATATTTTAACCACTTACCAAAATCTCTACT",
                "TCTTTTATATCCATAAATCCAACAACTCCTA",
                "CTCTCAAACATATATTTCTATAACTCTTATC",
                "ACAAATAATAAAACATCCATTTCATTCATAA",
                "CACCACCAAACCTTATAATCCCCAACCACAC",
            ])
        );
    }
}
