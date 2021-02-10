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

func main() {
	fmt.Println("hello world")
}

func edge(input string) string {
	return fmt.Sprintf("* %s *", input)
}

func bannerize(input string) (output [5]string) {
	output[0] = strings.Repeat("*", len(input) + 4)
	output[1] = edge(strings.Repeat(" ", len(input)))
	output[2] = edge(input)
	output[3] = output[1]
	output[4] = output[0]
	return
}
