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
	"time"
)

const romanAlphabet string = "abcdefghijklmnopqrstuvwxyz"
var zSeed = time.Now().UnixNano()

func main() {
	fmt.Println("hello world")
}

func getRandomLetters(count int) (randomLetters string) {
	for index := 0; index < count; index++ {
		randomLetters += string(romanAlphabet[rand.Intn(len(romanAlphabet))])
	}
	return
}

func padPlaintext(key int, plaintext string) (paddedPlaintext string) {
	var desiredSize int
	if key > len(plaintext) {
		desiredSize = key
	} else {
		desiredSize = len(plaintext) + ((key - len(plaintext) % key) % key)
	}
	return plaintext + getRandomLetters(desiredSize - len(plaintext))
}

func encrypt(key int, plaintext string) (ciphertext string) {
	paddedPlaintext := padPlaintext(key, plaintext)
	fmt.Println(paddedPlaintext)
	for column := 0; column < key; column++ {
		for characterIndex := column; characterIndex < len(paddedPlaintext); characterIndex += key {
			fmt.Println(characterIndex)
			ciphertext += string(paddedPlaintext[characterIndex])
		}
		fmt.Println(ciphertext)
	}
	return
}

func decrypt(key int, ciphertext string) string {
	return encrypt(len(ciphertext) / key, ciphertext)
}
