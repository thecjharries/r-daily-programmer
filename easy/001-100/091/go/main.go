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
	"time"
)

var zPrint = fmt.Println

func main() {
	_, _ = zPrint(sleepSort([]int{3, 1, 4, 1, 5, 9}))
}

func sleepSortRoutine(number int, sortChannel chan int) {
	time.Sleep(time.Millisecond * time.Duration(number))
	sortChannel <- number
}

func sleepSort(numbers []int) (sorted []int) {
	sortChannel := make(chan int, len(numbers))
	for _, number := range numbers {
		go sleepSortRoutine(number, sortChannel)
	}
	for index := 0; index < len(numbers); index++ {
		sorted = append(sorted, <-sortChannel)
	}
	return
}
