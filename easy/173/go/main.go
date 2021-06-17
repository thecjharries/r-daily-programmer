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
		"inches":      0.0,
		"miles":       0.0,
		"attoparsecs": 0.0,
	},
	"inches": {
		"metres":      0.0,
		"miles":       0.0,
		"attoparsecs": 0.0,
	},
	"miles": {
		"metres":      0.0,
		"inches":      0.0,
		"attoparsecs": 0.0,
	},
	"attoparsecs": {
		"metres": 0.0,
		"inches": 0.0,
		"miles":  0.0,
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
