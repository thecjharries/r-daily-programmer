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
	"math/rand"
)

var leetDictionary = map[rune][]string{
	'A': {"4", "/-\\", "/_\\", "@", "/\\", "Д", "а"},
	'B': {"8", "|3", "13", "|}", "|:", "|8", "18", "6", "|B", "|8", "lo", "|o", "j3", "ß", "в", "ь"},
	'C': {"<", "{", "[", "(", "©", "¢", "с"},
	'D': {"|)", "|}", "|]", "|>"},
	'E': {"3", "£", "₤", "€", "е"},
	'F': {"7", "|=", "ph", "|#", "|", "ƒ"},
	'G': {"[", "-", "[+", "6", "C-"},
	'H': {"#", "4", "|-|", "[-]", "{-}", "}-{", "}{", "|=|", "[=]", "{=}", "/-/", "(-)", ")-(", ":-:", "I+I", "н"},
	'I': {"1", "|", "!", "9"},
	'J': {"_|", "_/", "_7", "9", "_)", "_]", "_}"},
	'K': {"|<", "1<", "l<", "|{", "l{"},
	'L': {"|_", "|", "1", "]["},
	'M': {"44", "|\\/|", "^^", "/\\/\\", "/X\\", "[]\\/][", "[]V[]", "][\\\\//][", "(V)", "//.", ".\\\\", "N\\", "м"},
	'N': {"|\\|", "/\\/", "/V", "][\\\\][", "И", "и", "п"},
	'O': {"0", "()", "[]", "{}", "<>", "Ø", "oh", "Θ", "о", "ө"},
	'P': {"|o", "|O", "|>", "|*", "|°", "|D", "/o", "[]D", "|7", "р"},
	'Q': {"O_", "9", "(,)", "0", "kw"},
	'R': {"|2", "12", ".-", "|^", "l2", "Я", "®"},
	'S': {"5", "$", "§"},
	'T': {"7", "+", "7`", "'|' ", "`|` ", "~|~ ", "-|-", "']['", "т"},
	'U': {"|_|", "\\_\\", "/_/", "\\_/", "(_)", "[_]", "{_}"},
	'V': {"\\/"},
	'W': {"\\/\\/", "(/\\)", "\\^/", "|/\\|", "\\X/", "\\\\'", "'//", "VV", "\\_|_/", "\\\\//\\\\//", "Ш", "2u", "\\V/"},
	'X': {"%", "*", "><", "}{", ")(", "Ж"},
	'Y': {"`/", "¥", "\\|/", "Ч", "ү", "у"},
	'Z': {"2", "5", "7_", ">_", "(/)"},
}

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func translateLetter(letter rune) string {
	possibleTranslations, exists := leetDictionary[letter]
	if exists {
		return possibleTranslations[rand.Intn(len(possibleTranslations))]
	}
	return string(letter)
}
