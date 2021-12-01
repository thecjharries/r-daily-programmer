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

type Book struct {
	Width int
	Title string
}

type BookShelf struct {
	Width int
	Books []Book
}

func (b BookShelf) AvailableWidth() (available int) {
	available = b.Width
	for _, book := range b.Books {
		available -= book.Width
	}
	return
}

func (b *BookShelf) InsertBook(book Book) bool {
	fmt.Println(b.AvailableWidth(), book.Title, book.Width)
	if b.AvailableWidth() >= book.Width {
		b.Books = append(b.Books, book)
		return true
	}
	return false
}

type BookShelves []BookShelf

func (b *BookShelves) InsertBook(book Book) bool {
	for index, _ := range *b {
		shelf := (*b)[index]
		if shelf.InsertBook(book) {
			(*b)[index] = shelf
			return true
		}
	}
	return false
}

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}
