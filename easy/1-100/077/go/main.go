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

func main() {
	// the bonus, 35+, generates very fast but takes forever to print
	// writing them to a file would probably be better
	length := 10
	fmt.Println(Morse(length))
}

func generateMorseSequences(length int, seed string) (sequences []string) {
	if length == 0 {
		return []string{seed}
	} else if length < 0 {
		return []string{}
	}
	sequences = generateMorseSequences(length-1, seed+".")
	sequences = append(sequences, generateMorseSequences(length-2, seed+"-")...)
	return
}

func Morse(length int) []string {
	return generateMorseSequences(length, "")
}
