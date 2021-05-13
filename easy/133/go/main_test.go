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
	"testing"

	. "gopkg.in/check.v1"
)

func TestRootMain(t *testing.T) { TestingT(t) }

type MainSuite struct{}

var _ = Suite(&MainSuite{})

var printCallCount int
var printSpyContents string

func printSpy(a ...interface{}) (n int, err error) {
	printSpyContents = fmt.Sprint(a...)
	printCallCount++
	return
}

func (s *MainSuite) SetUpTest(c *C) {
	printCallCount = 0
	printSpyContents = ""
	zPrint = printSpy
}

func (s *MainSuite) TearDownTest(c *C) {
	zPrint = fmt.Println
}

func (s *MainSuite) TestMain(c *C) {
	c.Assert(printCallCount, Equals, 0)
	c.Assert(printSpyContents, Equals, "")
	main()
	c.Assert(printCallCount, Equals, 1)
	c.Assert(printSpyContents, Equals, "hello world")
}

func (s *MainSuite) TestBuildingProcessRoomLogEntry(c *C) {
	rooms := make(map[int]*RoomData)
	building := Building{Rooms: rooms}
	c.Assert(len(building.Rooms) == 0, Equals, true)
	building.ProcessRoomLogEntry(RoomLogEntry{0, 0, true, 540})
	c.Assert(len(building.Rooms) == 1, Equals, true)
	building.ProcessRoomLogEntry(RoomLogEntry{0, 0, false, 560})
	c.Assert(len(building.Rooms) == 1, Equals, true)
	c.Assert(building.Rooms[0].VisitorCount, Equals, 1)
	c.Assert(building.Rooms[0].TotalTime, Equals, 20)
}

func (s *MainSuite) TestNewBuilding(c *C) {
	entries := []RoomLogEntry{
		{0, 0, true, 540},
		{0, 0, false, 560},
	}
	building := NewBuilding(entries)
	c.Assert(building.Rooms[0].VisitorCount, Equals, 1)
	c.Assert(building.Rooms[0].TotalTime, Equals, 20)
}

func (s *MainSuite) TestRoomDataString(c *C) {
	room := RoomData{
		TotalTime:    40,
		VisitorCount: 2,
	}
	c.Assert(room.String(0), Equals, "Room 0, 20 minute average visit, 2 visitor(s) total")
}

func (s *MainSuite) TestBuildingString(c *C) {
	entries := []RoomLogEntry{
		{0, 0, true, 540},
		{0, 0, false, 560},
	}
	building := NewBuilding(entries)
	c.Assert(building.String(), Equals, "Room 0, 20 minute average visit, 1 visitor(s) total\n")
}
