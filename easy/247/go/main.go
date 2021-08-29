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
	"math/rand"
	"sort"
)

type Family []string
type SecretSantaList []Family

func (l SecretSantaList) Count() (count int) {
	for _, family := range l {
		count += len(family)
	}
	return count
}

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

func shuffleAndSortList(list SecretSantaList) (shuffledAndSorted SecretSantaList) {
	shuffledAndSorted = make(SecretSantaList, len(list))
	copy(shuffledAndSorted, list)
	for familyIndex, family := range shuffledAndSorted {
		rand.Shuffle(len(family), func(i, j int) { family[i], family[j] = family[j], family[i] })
		shuffledAndSorted[familyIndex] = family
	}
	rand.Shuffle(len(shuffledAndSorted), func(i, j int) {
		shuffledAndSorted[i], shuffledAndSorted[j] = shuffledAndSorted[j], shuffledAndSorted[i]
	})
	sort.Sort(shuffledAndSorted)
	return
}

func assignSecretSantas(list SecretSantaList) (results []string) {
	if 0 != list.Count()%2 {
		return
	}
	workingList := shuffleAndSortList(list)
	for 0 < len(workingList) {
		currentFamily := workingList[0]
		workingList = workingList[1:]
		nextFamily := 0
		for 0 < len(currentFamily) {
			results = append(results, fmt.Sprintf("%s -> %s", currentFamily[0], workingList[nextFamily][0]))
			currentFamily = currentFamily[1:]
			if 1 == len(workingList[nextFamily]) {
				workingList = append(workingList[:nextFamily], workingList[nextFamily+1:]...)
			} else {
				workingList[nextFamily] = workingList[nextFamily][1:]
			}
			if 0 < len(workingList) {
				nextFamily = (nextFamily + 1) % len(workingList)
			}
		}
	}
	return
}
