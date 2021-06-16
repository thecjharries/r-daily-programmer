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

// https://gist.github.com/anonymous/0ce707518d9e581499f5
var pbmDictionary = map[rune]string{
	'a': "0 0 1 0 0\n0 1 0 1 0\n1 0 0 0 1\n1 1 1 1 1\n1 0 0 0 1\n1 0 0 0 1\n1 0 0 0 1",
	'b': "1 1 1 1 0\n1 0 0 0 1\n1 0 0 0 1\n1 1 1 1 0\n1 0 0 0 1\n1 0 0 0 1\n1 1 1 1 0",
	'c': "0 1 1 1 0\n1 0 0 0 1\n1 0 0 0 0\n1 0 0 0 0\n1 0 0 0 0\n1 0 0 0 1\n0 1 1 1 0",
}

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}
