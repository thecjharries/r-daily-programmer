package main

import (
	"fmt"
	"strings"
)

// https://gist.github.com/mohayonao/094c71af14fe4791c5dd
var romanToMorse = map[string]string {
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
	" ": "/",
}

var morseToRoman = map[string]string {
	"-----": "0",
	".----": "1",
	"..---": "2",
	"...--": "3",
	"....-": "4",
	".....": "5",
	"-....": "6",
	"--...": "7",
	"---..": "8",
	"----.": "9",
	".-": "a",
	"-...": "b",
	"-.-.": "c",
	"-..": "d",
	".": "e",
	"..-.": "f",
	"--.": "g",
	"....": "h",
	"..": "i",
	".---": "j",
	"-.-": "k",
	".-..": "l",
	"--": "m",
	"-.": "n",
	"---": "o",
	".--.": "p",
	"--.-": "q",
	".-.": "r",
	"...": "s",
	"-": "t",
	"..-": "u",
	"...-": "v",
	".--": "w",
	"-..-": "x",
	"-.--": "y",
	"--..": "z",
	".-.-.-": ".",
	"--..--": ",",
	"..--..": "?",
	"-.-.--": "!",
	"-....-": "-",
	"-..-.": "/",
	".--.-.": "@",
	"-.--.": "(",
	"-.--.-": ")",
	"/": " ",
}

const (
	errorNotSingleCharacter string = "Expected a single character; got multiple"
	errorUnknownCharacter string = "Unknown Roman character"
)

func main() {
	fmt.Println("hello world")
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
	input := strings.Split(morseWords, " ")
	output := ""
	for _, character := range input {
		output += morseCharacterToRomanCharacter(character)
	}
	return output
}
