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

var alphabet string = "abcdefghijklmnopqrstuvwxyz"

type GenericCipher interface {
	Encrypt(input string) string
	Decrypt(input string) string
}

type CaesarCipher struct {
	places          int
	encryptAlphabet string
	decryptAlphabet string
}

func (c CaesarCipher) Encrypt(input string) string {
	return translateString(input, c.encryptAlphabet)
}

func (c CaesarCipher) Decrypt(input string) string {
	return translateString(input, c.decryptAlphabet)
}

func NewCaesarCipher(places int) *CaesarCipher {
	cipher := CaesarCipher{places: places}
	cipher.encryptAlphabet = rotateAlphabet(places)
	cipher.decryptAlphabet = rotateAlphabet(-places)
	return &cipher
}

func main() {
	input := "uftu"
	places := 1
	cipher := NewCaesarCipher(places)
	encrypted := cipher.Encrypt(input)
	decrypted := cipher.Decrypt(input)
	fmt.Printf("New alphabet: %s\n", cipher.encryptAlphabet)
	fmt.Printf("Input: %s\n", input)
	fmt.Printf("Encrypted input: %s\n", encrypted)
	fmt.Printf("Decrypted input: %s\n", decrypted)
}

func rotateAlphabet(places int) string {
	shift := ((places % len(alphabet)) + len(alphabet)) % len(alphabet)
	return alphabet[shift:] + alphabet[:shift]
}

func translateCharacter(character, newAlphabet string) string {
	characterIndex := strings.Index(alphabet, character)
	if -1 < characterIndex {
		return string(newAlphabet[characterIndex])
	}
	return ""
}

func translateString(input, newAlphabet string) string {
	result := ""
	for _, character := range input {
		result += translateCharacter(string(character), newAlphabet)
	}
	return result
}
