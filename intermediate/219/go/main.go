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

type TodoList []string

func (l *TodoList) AddItem(item string) *TodoList {
	*l = append(*l, item)
	return l
}

func (l *TodoList) DeleteItem(item string) *TodoList {
	for index, element := range *l {
		if item == element {
			*l = append((*l)[:index], (*l)[index+1:]...)
			break
		}
	}
	return l
}

func (l *TodoList) UpdateItem(item, updatedItem string) *TodoList {
	for index, element := range *l {
		if item == element {
			(*l)[index] = updatedItem
			break
		}
	}
	return l
}

func (l *TodoList) ViewList() string {
	return strings.Join(*l, "\n")
}

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}
