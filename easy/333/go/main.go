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
	"strings"
)

var packetPattern = regexp.MustCompile(`^(\d+)\s+(\d+)\s+(\d+)\s*(.*)$`)

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func assemblePackets(packets []string) (messages map[int]string) {
	rawMessages := make(map[int][]string)
	for _, packet := range packets {
		fmt.Println(packet)
		matches := packetPattern.FindStringSubmatch(packet)
		packetId, _ := strconv.Atoi(matches[1])
		packetChunk, _ := strconv.Atoi(matches[2])
		totalChunks, _ := strconv.Atoi(matches[3])
		currentChunks, exists := rawMessages[packetId]
		if !exists {
			currentChunks = make([]string, totalChunks)
		}
		currentChunks[packetChunk] = matches[4]
		rawMessages[packetId] = currentChunks
	}
	messages = make(map[int]string)
	for packetId, packetChunks := range rawMessages {
		messages[packetId] = strings.Join(packetChunks, "")
	}
	return
}
