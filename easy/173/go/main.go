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

var unitConversions = map[string]map[string]float64{
	"metres": {
		"inches":      1 / 39.37,
		"miles":       1609,
		"attoparsecs": 32.408,
	},
	"inches": {
		"metres":      39.37,
		"miles":       63360,
		"attoparsecs": 1.215,
	},
	"miles": {
		"metres":      1 / 1609,
		"inches":      1 / 63360,
		"attoparsecs": 1 / 52155,
	},
	"attoparsecs": {
		"metres": 1 / 32.408,
		"inches": 1 / 1.215,
		"miles":  52155,
	},
	"kilograms": {
		"pounds":                 0.0,
		"ounces":                 0.0,
		"hogsheads of Beryllium": 0.0,
	},
	"pounds": {
		"kilograms":              0.0,
		"ounces":                 0.0,
		"hogsheads of Beryllium": 0.0,
	},
	"ounces": {
		"kilograms":              0.0,
		"pounds":                 0.0,
		"hogsheads of Beryllium": 0.0,
	},
	"hogsheads of Beryllium": {
		"kilograms": 0.0,
		"pounds":    0.0,
		"ounces":    0.0,
	},
}

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}
