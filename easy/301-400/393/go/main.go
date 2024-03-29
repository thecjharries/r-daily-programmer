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

var values = []int{1, 5, 10, 25, 100, 500}

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func change(input int) (totalCoins int) {
	for index := len(values) - 1; index >= 0; index-- {
		if input >= values[index] {
			totalCoins += input / values[index]
			input = input % values[index]
		}
	}
	return
}
