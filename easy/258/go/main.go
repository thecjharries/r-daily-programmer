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
	"net"
)

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func pingIrcNode() {
	connection, _ := net.Dial("tcp", "chat.freenode.net:6667")
	defer (func() { _ = connection.Close() })()
	_, _ = connection.Write([]byte("NICK wotw_test"))
	_, _ = connection.Write([]byte("USER wotw_test 0 :CJ"))
	readBuffer := make([]byte, 2048)
	count, _ := connection.Read(readBuffer)
	fmt.Println(string(readBuffer[:count]))
}
