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

var sevenSegmentToDigit = map[string]int{
	"   \n  |\n  |": 1,
	" _ \n _|\n|_ ": 2,
	" _ \n _|\n _|": 3,
	"   \n|_|\n  |": 4,
	" _ \n|_ \n _|": 5,
	" _ \n|_ \n|_|": 6,
	" _ \n  |\n  |": 7,
	" _ \n|_|\n|_|": 8,
	" _ \n|_|\n _|": 9,
}

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}
