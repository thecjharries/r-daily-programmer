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
	"strings"
)

var functionMap = map[string]string {
	"sqrt": "sqrtf",
	"abs": "fabsf",
	"sin": "sinf",
	"cos": "cosf",
	"tan": "tanf",
	"exp": "expf",
	"log": "logf",
}

func main() {
	fmt.Println("hello world")
}

func splitEquation(equation string) (string, string) {
	exploded := strings.Split(equation, "=")
	return exploded[0], exploded[1]
}

func sanitizeRhs(rhs string) string {
	sanitized := rhs
	for simpleFunc, properFunc := range functionMap {
		sanitized = strings.ReplaceAll(sanitized, simpleFunc, properFunc)
	}
	return sanitized
}
