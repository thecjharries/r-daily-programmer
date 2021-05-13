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

type RoomData struct {
	TotalTime    int
	VisitorCount int
}

func (r *RoomData) String(roomId int) string {
	return fmt.Sprintf(
		"Room %d, %d minute average visit, %d visitor(s) total",
		roomId,
		r.TotalTime/r.VisitorCount,
		r.VisitorCount,
	)
}

type Building struct {
	Rooms map[int]*RoomData
}

type RoomLogEntry struct {
	VisitorId int
	RoomId    int
	Enter     bool
	Timestamp int
}

func (b *Building) ProcessRoomLogEntry(entry RoomLogEntry) {
	var timeIncrement, visitorIncrement int
	if entry.Enter {
		timeIncrement = -entry.Timestamp
		visitorIncrement = 1
	} else {
		timeIncrement = entry.Timestamp
		visitorIncrement = 0
	}
	_, exists := b.Rooms[entry.RoomId]
	if exists {
		b.Rooms[entry.RoomId].TotalTime += timeIncrement
		b.Rooms[entry.RoomId].VisitorCount += visitorIncrement
	} else {
		b.Rooms[entry.RoomId] = &RoomData{
			TotalTime:    timeIncrement,
			VisitorCount: visitorIncrement,
		}
	}
}

func NewBuilding(roomLogEntries []RoomLogEntry) Building {
	rooms := make(map[int]*RoomData)
	building := Building{Rooms: rooms}
	for _, entry := range roomLogEntries {
		building.ProcessRoomLogEntry(entry)
	}
	return building
}

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}
