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
	"regexp"
	"strconv"
)

var semverPattern = regexp.MustCompile(`^(?P<major>\d+)\.(?P<minor>\d+).(?P<patch>\d+)(?:-(?P<label>[^+\s]+))?(?:\+.*)?$`)

type SemVer struct {
	Major, Minor, Patch int
	Label               string
}

func NewSemVer(input string) SemVer {
	match := semverPattern.FindAllStringSubmatch(input, -1)[0]
	major, _ := strconv.Atoi(match[1])
	minor, _ := strconv.Atoi(match[2])
	patch, _ := strconv.Atoi(match[3])
	return SemVer{
		Major: major,
		Minor: minor,
		Patch: patch,
		Label: match[4],
	}
}

type SemVers []SemVer

func (s SemVers) Length() int {
	return len(s)
}

func (s SemVers) Swap(i, j int) {
	s[i], s[j] = s[j], s[i]
}

func (s SemVers) Less(i, j int) bool {
	if s[i].Major < s[j].Major {
		return true
	} else if s[i].Major > s[j].Major {
		return false
	}
	if s[i].Minor < s[j].Minor {
		return true
	} else if s[i].Minor > s[j].Minor {
		return false
	}
	if s[i].Patch < s[j].Patch {
		return true
	} else if s[i].Patch > s[j].Patch {
		return false
	}
	return "" != s[i].Label && "" == s[j].Label
}

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}
