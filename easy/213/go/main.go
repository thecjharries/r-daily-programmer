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

var highOrderHexToWord = map[rune]string{
	'1': "Eleventy",
	'2': "Twenty",
	'3': "Thirty",
	'4': "Fourty",
	'5': "Fifty",
	'6': "Sitxy",
	'7': "Seventy",
	'8': "Eighty",
	'9': "Ninety",
	'A': "Atta",
	'B': "Bibbity",
	'C': "City",
	'D': "Dickety",
	'E': "Ebbity",
	'F': "Fleventy",
}

var lowOrderHexToWord = map[rune]string{
	'1': "One",
	'2': "Two",
	'3': "Three",
	'4': "Four",
	'5': "Five",
	'6': "Six",
	'7': "Seven",
	'8': "Eight",
	'9': "Nine",
	'A': "Aye",
	'B': "Bee",
	'C': "Cee",
	'D': "Dee",
	'E': "Ee",
	'F': "Eff",
}

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}
