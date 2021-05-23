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

package main

import "fmt"

var brailleToRoman = map[string]string{
	"O.....": "a",
	"O.O...": "b",
	"OO....": "c",
	"OO.O..": "d",
	"O..O..": "e",
	"OOO...": "f",
	"OOOO..": "g",
	"O.OO..": "h",
	".OO...": "i",
	".OOO..": "j",
	"O...O.": "k",
	"O.O.O.": "l",
	"OO..O.": "m",
	"OO.OO.": "n",
	"O..OO.": "o",
	"OOO.O.": "p",
	"OOOOO.": "q",
	"O.OOO.": "r",
	".OO.O.": "s",
	".OOOO.": "t",
	"O...OO": "u",
	"O.O.OO": "v",
	".OOO.O": "w",
	"OO..OO": "x",
	"OO.OOO": "y",
	"O..OOO": "z",
}

var romanToBraille = map[string]string{
	"a": "O.....",
	"b": "O.O...",
	"c": "OO....",
	"d": "OO.O..",
	"e": "O..O..",
	"f": "OOO...",
	"g": "OOOO..",
	"h": "O.OO..",
	"i": ".OO...",
	"j": ".OOO..",
	"k": "O...O.",
	"l": "O.O.O.",
	"m": "OO..O.",
	"n": "OO.OO.",
	"o": "O..OO.",
	"p": "OOO.O.",
	"q": "OOOOO.",
	"r": "O.OOO.",
	"s": ".OO.O.",
	"t": ".OOOO.",
	"u": "O...OO",
	"v": "O.O.OO",
	"w": ".OOO.O",
	"x": "OO..OO",
	"y": "OO.OOO",
	"z": "O..OOO",
}

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func convertBrailleToRoman(brailleLetters []string) (romanLetters string) {
	for _, letter := range brailleLetters {
		foundLetter, exists := brailleToRoman[letter]
		if exists {
			romanLetters += foundLetter
		}
	}
	return
}
