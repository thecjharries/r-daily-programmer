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

fn build_presents_song(presents: Vec<String>) -> String {
    todo!()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(
            "On the first day of Christmas\nmy true love sent to me:\n1 Partridge in a Pear Tree\n\nOn the second day of Christmas\nmy true love sent to me:\n2 Turtle Doves\nand 1 Partridge in a Pear Tree\n\nOn the third day of Christmas\nmy true love sent to me:\n3 French Hens\n2 Turtle Doves\nand 1 Partridge in a Pear Tree\n\nOn the fourth day of Christmas\nmy true love sent to me:\n4 Calling Birds\n3 French Hens\n2 Turtle Doves\nand 1 Partridge in a Pear Tree\n\nOn the fifth day of Christmas\nmy true love sent to me:\n5 Golden Rings\n4 Calling Birds\n3 French Hens\n2 Turtle Doves\nand 1 Partridge in a Pear Tree\n\nOn the sixth day of Christmas\nmy true love sent to me:\n6 Geese a Laying\n5 Golden Rings\n4 Calling Birds\n3 French Hens\n2 Turtle Doves\nand 1 Partridge in a Pear Tree\n\nOn the seventh day of Christmas\nmy true love sent to me:\n7 Swans a Swimming\n6 Geese a Laying\n5 Golden Rings\n4 Calling Birds\n3 French Hens\n2 Turtle Doves\nand 1 Partridge in a Pear Tree\n\nOn the eighth day of Christmas\nmy true love sent to me:\n8 Maids a Milking\n7 Swans a Swimming\n6 Geese a Laying\n5 Golden Rings\n4 Calling Birds\n3 French Hens\n2 Turtle Doves\nand 1 Partridge in a Pear Tree\n\nOn the ninth day of Christmas\nmy true love sent to me:\n9 Ladies Dancing\n8 Maids a Milking\n7 Swans a Swimming\n6 Geese a Laying\n5 Golden Rings\n4 Calling Birds\n3 French Hens\n2 Turtle Doves\nand 1 Partridge in a Pear Tree\n\nOn the tenth day of Christmas\nmy true love sent to me:\n10 Lords a Leaping\n9 Ladies Dancing\n8 Maids a Milking\n7 Swans a Swimming\n6 Geese a Laying\n5 Golden Rings\n4 Calling Birds\n3 French Hens\n2 Turtle Doves\nand 1 Partridge in a Pear Tree\n\nOn the eleventh day of Christmas\nmy true love sent to me:\n11 Pipers Piping\n10 Lords a Leaping\n9 Ladies Dancing\n8 Maids a Milking\n7 Swans a Swimming\n6 Geese a Laying\n5 Golden Rings\n4 Calling Birds\n3 French Hens\n2 Turtle Doves\nand 1 Partridge in a Pear Tree\n\nOn the twelfth day of Christmas\nmy true love sent to me:\n12 Drummers Drumming\n11 Pipers Piping\n10 Lords a Leaping\n9 Ladies Dancing\n8 Maids a Milking\n7 Swans a Swimming\n6 Geese a Laying\n5 Golden Rings\n4 Calling Birds\n3 French Hens\n2 Turtle Doves\nand 1 Partridge in a Pear Tree".to_string(),
            build_presents_song(vec![
                "Partridge in a Pear Tree".to_string(),
                "Turtle Doves".to_string(),
                "French Hens".to_string(),
                "Calling Birds".to_string(),
                "Golden Rings".to_string(),
                "Geese a Laying".to_string(),
                "Swans a Swimming".to_string(),
                "Maids a Milking".to_string(),
                "Ladies Dancing".to_string(),
                "Lords a Leaping".to_string(),
                "Pipers Piping".to_string(),
                "Drummers Drumming".to_string(),
            ])
        );
    }
}
