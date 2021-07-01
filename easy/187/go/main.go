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

type Flag struct {
	Short, Long string
}

func (f *Flag) IsActive(input string) bool {
	return f.Short == input || f.Long == input
}

func (f *Flag) String() string {
	return fmt.Sprintf("flag: %s", f.Long)
}

func NewFlag(short, long string) *Flag {
	return &Flag{
		Short: short,
		Long:  long,
	}
}

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func parseFlags(input string, availableFlags []*Flag) (output string) {
	argv := strings.Split(input, " ")
	for _, arg := range argv {
		if strings.HasPrefix(arg, "--") {
			currentArg := strings.TrimPrefix(arg, "--")
			found := false
			for _, flag := range availableFlags {
				if flag.IsActive(currentArg) {
					output += flag.String() + "\n"
					found = true
					break
				}
			}
			if !found {
				output += fmt.Sprintf("flag: %s\n", currentArg)
			}
		} else if strings.HasPrefix(arg, "-") {
			for _, shortFlag := range strings.Split(strings.TrimPrefix(arg, "-"), "") {
				for _, flag := range availableFlags {
					if flag.IsActive(shortFlag) {
						output += flag.String() + "\n"
						break
					}
				}
			}
		} else {
			output += fmt.Sprintf("parameter: %s\n", arg)
		}
	}
	return
}
