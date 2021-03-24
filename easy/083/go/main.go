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

type NumberScale struct {
	Number uint64
	Short  string
	Long   string
}

var ScalesLargestToSmallest = []NumberScale{
	{1000000000000000000000, "sextillion", "trilliard"},
	{1000000000000000000, "quintillion", "trillion"},
	{1000000000000000, "quadrillion", "billiard"},
	{1000000000000, "trillion", "billion"},
	{1000000000, "billion", "milliard"},
	{1000000, "million", "million"},
}

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}
