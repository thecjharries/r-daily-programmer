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

fn is_palindrome(input: &str) -> bool {
    for index in 0..input.len() / 2 {
        if input.chars().nth(index).unwrap() != input.chars().nth(input.len() - index - 1).unwrap()
        {
            return false;
        }
    }
    true
}

fn find_longest_palindrome(text: &str) -> String {
    todo!()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        assert!(is_palindrome("radar"));
        assert!(is_palindrome("racecar"));
        assert!(!is_palindrome("palindrome"));
    }

    #[test]
    fn test_find_longest_palindrome() {
        assert_eq!(
            "racecar",
            find_longest_palindrome("I like racecars that go fast")
        );
        assert_eq!("aibohphobia", find_longest_palindrome("aibohphobia"));
        assert_eq!(
            "racer",
            find_longest_palindrome("My dad is a racecar athlete")
        );
        assert_eq!(
            "ranynar",
            find_longest_palindrome("Fourscoreandsevenyearsagoourfaathersbroughtforthonthisconta inentanewnationconceivedinzLibertyanddedicatedtotheproposit ionthatallmenarecreatedequalNowweareengagedinagreahtcivilwa rtestingwhetherthatnaptionoranynartionsoconceivedandsodedic atedcanlongendureWeareqmetonagreatbattlefiemldoftzhatwarWeh avecometodedicpateaportionofthatfieldasafinalrestingplacefo rthosewhoheregavetheirlivesthatthatnationmightliveItisaltog etherfangandproperthatweshoulddothisButinalargersensewecann otdedicatewecannotconsecratewecannothallowthisgroundThebrav elmenlivinganddeadwhostruggledherehaveconsecrateditfarabove ourpoorponwertoaddordetractTgheworldadswfilllittlenotlenorl ongrememberwhatwesayherebutitcanneverforgetwhattheydidhereI tisforusthelivingrathertobededicatedheretotheulnfinishedwor kwhichtheywhofoughtherehavethusfarsonoblyadvancedItisrather forustobeherededicatedtothegreattdafskremainingbeforeusthat fromthesehonoreddeadwetakeincreaseddevotiontothatcauseforwh ichtheygavethelastpfullmeasureofdevotionthatweherehighlyres olvethatthesedeadshallnothavediedinvainthatthisnationunsder Godshallhaveanewbirthoffreedomandthatgovernmentofthepeopleb ythepeopleforthepeopleshallnotperishfromtheearth");
        )
    }
}
