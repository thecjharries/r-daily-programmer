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
	"reflect"

	"github.com/romanyx/jwalk"
)

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func findJsonItem(itemToFind string, rawJson string) (result string) {
	walkableJson, _ := jwalk.Parse([]byte(rawJson))
	switch rootJson := walkableJson.(type) {
	case jwalk.ObjectWalker:
		_ = rootJson.Walk(func(key string, value interface{}) error {
			switch parsedJsonValue := value.(type) {
			case jwalk.Value:
				switch parsedValue := parsedJsonValue.Interface().(type) {
				case []interface{}:
					for index, element := range parsedValue {
						switch item := element.(type) {
						case string:
							if item == itemToFind {
								result = fmt.Sprintf("%s -> %d", key, index)
							}
						}
					}
				default:
					fmt.Println(reflect.TypeOf(parsedValue))
				}
			}
			return nil
		})
	}
	return
}
