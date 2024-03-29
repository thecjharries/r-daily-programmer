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

type ForkInputFunc func(int, ...int) int

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func forkFunc(first, second ForkInputFunc, remaining ...ForkInputFunc) ForkInputFunc {
	if 0 < len(remaining) {
		if 1 == len(remaining) {
			return func(y int, x ...int) int {
				return second(first(y, x...), remaining[0](y, x...))
			}
		} else if 1 == len(remaining)%2 {
			return func(y int, x ...int) int {
				return second(first(y, x...), forkFunc(remaining[0], remaining[1], remaining[2:]...)(y, x...))
			}
		}
	}
	return nil
}

func sum(y int, x ...int) (totalSum int) {
	totalSum = y
	for _, number := range x {
		totalSum += number
	}
	return
}

func count(y int, x ...int) int {
	paramCount := y
	paramCount = len(x) + 1
	return paramCount
}

func divide(y int, x ...int) int {
	return y / x[0]
}
