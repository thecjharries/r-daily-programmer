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

const romanAlphabet string = "abcdefghijklmnopqrstuvwxyz"

func main() {
	encryptedString := `    Spzalu - zayhunl dvtlu sfpun pu wvukz kpzaypibapun zdvykz pz uv ihzpz mvy h
    zfzalt vm nvclyutlua.  Zbwyltl leljbapcl wvdly klypclz myvt h thukhal myvt aol
    thzzlz, uva myvt zvtl mhyjpjhs hxbhapj jlyltvuf. Fvb jhu'a lewlja av dplsk
    zbwyltl leljbapcl wvdly qbza 'jhbzl zvtl dhalyf ahya aoyld h zdvyk ha fvb! P
    tlhu, pm P dlua hyvbuk zhfpu' P dhz hu ltwlylyvy qbza iljhbzl zvtl tvpzalulk
    ipua ohk sviilk h zjptpahy ha tl aolf'k wba tl hdhf!... Ho, huk uvd dl zll aol
    cpvslujl puolylua pu aol zfzalt! Jvtl zll aol cpvslujl puolylua pu aol zfzalt!
    Olsw! Olsw! P't ilpun ylwylzzlk!`
	for index := 0; index < 26; index++ {
		fmt.Printf("Decrypting with a shift of %d\n%s\n\n", index, decryptString(encryptedString, index))
	}
}

func sanitizeShift(shift int) int {
	return ((shift % 26) + 26) % 26
}

func encryptLetter(letter string, shift int) string {
	inputIndex := strings.Index(romanAlphabet, strings.ToLower(letter))
	if inputIndex < 0 {
		return letter
	}
	outputIndex := sanitizeShift(inputIndex + shift)
	return string(romanAlphabet[outputIndex])
}

func decryptLetter(letter string, shift int) string {
	return encryptLetter(letter, 26-shift)
}

func encryptString(input string, shift int) string {
	output := ""
	for _, character := range input {
		output += encryptLetter(string(character), shift)
	}
	return output
}

func decryptString(input string, shift int) string {
	return encryptString(input, 26-shift)
}
