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
	"strings"
)

const tetrominoPieces string = "OISZLJT"

type Bag []string

func NewBag() Bag {
	pieces := strings.Split(tetrominoPieces, "")
	rand.Shuffle(len(pieces), func(i, j int) {
		pieces[i], pieces[j] = pieces[j], pieces[i]
	})
	return Bag(pieces)
}

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func buildGameSet(numberOfPieces int) (result string) {
	for index := 0; index < numberOfPieces/len(tetrominoPieces); index++ {
		bag := NewBag()
		result += strings.Join(bag, "")
	}
	remainingBag := NewBag()
	result += strings.Join(remainingBag[:numberOfPieces%7], "")
	return
}
