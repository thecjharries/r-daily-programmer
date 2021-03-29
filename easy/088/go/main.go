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
	"regexp"
	"strings"
)

const romanAlphabet = "abcdefghijklmnopqrstuvwxyz"

var notLettersPattern = regexp.MustCompile(`[^a-z]`)

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func sanitizePlaintext(plaintext string) string {
	sanitized := strings.ToLower(plaintext)
	return notLettersPattern.ReplaceAllString(sanitized, "")
}

func encrypt(key, plaintext string) (ciphertext string) {
	sanitizedKey := sanitizePlaintext(key)
	sanitizedPlaintext := sanitizePlaintext(plaintext)
	for index, character := range sanitizedPlaintext {
		keyIndex := strings.Index(romanAlphabet, string(sanitizedKey[index%len(sanitizedKey)]))
		characterIndex := strings.Index(romanAlphabet, string(character))
		ciphertext += string(romanAlphabet[(keyIndex+characterIndex)%len(romanAlphabet)])
	}
	return
}

func decrypt(key, ciphertext string) (plaintext string) {
	sanitizedKey := sanitizePlaintext(key)
	sanitizedCipherText := sanitizePlaintext(ciphertext)
	for index, character := range sanitizedCipherText {
		keyIndex := strings.Index(romanAlphabet, string(sanitizedKey[index%len(sanitizedKey)]))
		characterIndex := strings.Index(romanAlphabet, string(character))
		plaintext += string(romanAlphabet[(26+characterIndex-keyIndex)%len(romanAlphabet)])
	}
	return
}
