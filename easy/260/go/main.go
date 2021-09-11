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

var doorStateMap = map[string]map[string]string{
	"closed": {
		"button_clicked": "opening",
		"cycle_complete": "closed",
	},
	"opening": {
		"button_clicked": "stopped_while_opening",
		"cycle_complete": "open",
	},
	"open": {
		"button_clicked": "closing",
		"cycle_complete": "open",
	},
	"closing": {
		"button_clicked": "stopped_while_closing",
		"cycle_complete": "closed",
	},
	"stopped_while_opening": {
		"button_clicked": "closing",
		"cycle_complete": "stopped_while_opening",
	},
	"stopped_while_closing": {
		"button_clicked": "opening",
		"cycle_complete": "stopped_while_opening",
	},
}

const startingState string = "closed"

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}
