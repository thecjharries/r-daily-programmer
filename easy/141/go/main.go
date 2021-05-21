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

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func computeFletcher16(data []byte) uint16 {
	var simpleSum, modularSum uint16
	for index := 0; index < len(data); index++ {
		simpleSum = (simpleSum + uint16(data[index])) % 255
		modularSum = (simpleSum + modularSum) % 255
	}
	return (modularSum << 8) | simpleSum
}
