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

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func bogo(scrambled, sorted string) (iterations int) {
	currentIteration := scrambled
	for currentIteration != sorted {
		exploded := strings.Split(currentIteration, "")
		rand.Shuffle(
			len(exploded),
			func(i, j int) {
				exploded[i], exploded[j] = exploded[j], exploded[i]
			},
		)
		currentIteration = strings.Join(exploded, "")
		iterations++
	}
	return
}
