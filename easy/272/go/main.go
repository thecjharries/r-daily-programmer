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

type ScrabbleTiles map[string]int

var EnglishScrabbleTiles = ScrabbleTiles{
	"A": 9,
	"B": 2,
	"C": 2,
	"D": 4,
	"E": 12,
	"F": 2,
	"G": 3,
	"H": 2,
	"I": 9,
	"J": 1,
	"K": 1,
	"L": 4,
	"M": 2,
	"N": 6,
	"O": 8,
	"P": 2,
	"Q": 1,
	"R": 6,
	"S": 4,
	"T": 6,
	"U": 4,
	"V": 2,
	"W": 2,
	"X": 1,
	"Y": 2,
	"Z": 1,
	"_": 2,
}

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}
