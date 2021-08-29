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
	"sort"
)

type Family []string
type SecretSantaList []Family

func (l SecretSantaList) Len() int {
	return len(l)
}

func (l SecretSantaList) Swap(i, j int) {
	l[i], l[j] = l[j], l[i]
}

func (l SecretSantaList) Less(i, j int) bool {
	return len(l[i]) > len(l[j])
}

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func assignSecretSantas(list SecretSantaList) (results []string) {
	workingList := make(SecretSantaList, len(list))
	copy(workingList, list)
	sort.Sort(workingList)
	for 0 < len(workingList) {
		currentFamily := workingList[0]
		workingList = workingList[1:]
		nextFamily := 0
		for 0 < len(currentFamily) {
			results = append(results, fmt.Sprintf("%s -> %s"), currentFamily[0], workingList[nextFamily][0])
			currentFamily = currentFamily[1:]
			if 1 == len(workingList[nextFamily]) {
				workingList = append(workingList[:nextFamily], workingList[nextFamily+1:]...)
			} else {
				workingList[nextFamily] = workingList[nextFamily][1:]
			}
			nextFamily = (nextFamily + 1) % len(workingList)
		}
	}
}
