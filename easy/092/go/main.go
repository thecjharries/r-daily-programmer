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

import (
	"fmt"
	"strings"
)

// https://gist.github.com/mohayonao/094c71af14fe4791c5dd
var romanToMorse = map[string]string{
	"0": "-----",
	"1": ".----",
	"2": "..---",
	"3": "...--",
	"4": "....-",
	"5": ".....",
	"6": "-....",
	"7": "--...",
	"8": "---..",
	"9": "----.",
	"a": ".-",
	"b": "-...",
	"c": "-.-.",
	"d": "-..",
	"e": ".",
	"f": "..-.",
	"g": "--.",
	"h": "....",
	"i": "..",
	"j": ".---",
	"k": "-.-",
	"l": ".-..",
	"m": "--",
	"n": "-.",
	"o": "---",
	"p": ".--.",
	"q": "--.-",
	"r": ".-.",
	"s": "...",
	"t": "-",
	"u": "..-",
	"v": "...-",
	"w": ".--",
	"x": "-..-",
	"y": "-.--",
	"z": "--..",
	".": ".-.-.-",
	",": "--..--",
	"?": "..--..",
	"!": "-.-.--",
	"-": "-....-",
	"/": "-..-.",
	"@": ".--.-.",
	"(": "-.--.",
	")": "-.--.-",
	" ": "",
}

var morseToRoman = map[string]string{
	"-----":  "0",
	".----":  "1",
	"..---":  "2",
	"...--":  "3",
	"....-":  "4",
	".....":  "5",
	"-....":  "6",
	"--...":  "7",
	"---..":  "8",
	"----.":  "9",
	".-":     "a",
	"-...":   "b",
	"-.-.":   "c",
	"-..":    "d",
	".":      "e",
	"..-.":   "f",
	"--.":    "g",
	"....":   "h",
	"..":     "i",
	".---":   "j",
	"-.-":    "k",
	".-..":   "l",
	"--":     "m",
	"-.":     "n",
	"---":    "o",
	".--.":   "p",
	"--.-":   "q",
	".-.":    "r",
	"...":    "s",
	"-":      "t",
	"..-":    "u",
	"...-":   "v",
	".--":    "w",
	"-..-":   "x",
	"-.--":   "y",
	"--..":   "z",
	".-.-.-": ".",
	"--..--": ",",
	"..--..": "?",
	"-.-.--": "!",
	"-....-": "-",
	"-..-.":  "/",
	".--.-.": "@",
	"-.--.":  "(",
	"-.--.-": ")",
	"  ":     " ",
}

const (
	errorNotSingleCharacter string = "Expected a single character; got multiple"
	errorUnknownCharacter   string = "Unknown Roman character"
)

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func romanCharacterToMorseCharacter(romanCharacter string) string {
	if 1 < len(romanCharacter) {
		panic(errorNotSingleCharacter)
	}
	value, exists := romanToMorse[romanCharacter]
	if !exists {
		panic(errorUnknownCharacter)
	}
	return value
}

func morseCharacterToRomanCharacter(morseCharacter string) string {
	value, exists := morseToRoman[morseCharacter]
	if !exists {
		panic(errorUnknownCharacter)
	}
	return value
}

func translateRomanToMorse(romanWords string) string {
	var output []string
	for _, character := range romanWords {
		output = append(output, romanCharacterToMorseCharacter(string(character)))
	}
	return strings.Join(output[:], " ")
}

func translateMorseToRoman(morseWords string) string {
	input := strings.Split(morseWords, "  ")
	output := ""

	firstWord := strings.Split(input[0], " ")
	for _, character := range firstWord {
		output += morseCharacterToRomanCharacter(character)
	}
	for _, word := range input[1:] {
		output += " "
		morseCharacters := strings.Split(word, " ")
		for _, character := range morseCharacters {
			output += morseCharacterToRomanCharacter(character)
		}
	}
	return output
}
