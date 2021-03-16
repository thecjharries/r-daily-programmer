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
	"regexp"
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

var lhsPattern = regexp.MustCompile(`(?P<name>[a-zA-Z0-9_]+)\((?P<args>[^)]+)\)`)

const funcTemplate string = `float %s
{
	return %s;
}`

func main() {
	fmt.Println(processEquation("big(x,y)=sqrt(x+y)*10"))
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

func sanitizeLhs(lhs string) string {
	matched := lhsPattern.FindStringSubmatch(lhs)
	args := strings.Split(matched[2], ",")
	for index := 0; index < len(args); index++ {
		args[index] = fmt.Sprintf("float %s", args[index])
	}
	return fmt.Sprintf("%s(%s)", matched[1], strings.Join(args, ", "))
}

func processEquation(equation string) string {
	lhs, rhs := splitEquation(equation)
	return fmt.Sprintf(funcTemplate, sanitizeLhs(lhs), sanitizeRhs(rhs))
}
