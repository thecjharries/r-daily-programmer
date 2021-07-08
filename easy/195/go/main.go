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

type PathResolver struct {
	Symlinks map[string]string
}

func (r *PathResolver) Resolve(path string) (resolvedPath string) {
	sections := strings.Split(path, "/")
	for _, element := range sections[1:] {
		resolvedPath = strings.Join([]string{resolvedPath, element}, "/")
		source, exists := r.Symlinks[resolvedPath]
		for exists {
			resolvedPath = source
			source, exists = r.Symlinks[resolvedPath]
		}
	}
	return
}

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}
