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
	"encoding/json"
	"fmt"
	"io/ioutil"
	"path"
	"regexp"
	"strconv"
)

var postTitlePattern = regexp.MustCompile(`.*Challenge #(\d+) \[(\w+)].*`)

type RawPost struct {
	Title     string                 `json:"title"`
	Url       string                 `json:"url"`
	Remaining map[string]interface{} `json:"-"`
}

type Post struct {
	Title      string
	Url        string
	Difficulty string
	Number     int
}

type Posts []Post

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func loadData() []RawPost {
	contents, _ := ioutil.ReadFile(path.Join("..", "simplified.json"))
	var posts []RawPost
	_ = json.Unmarshal(contents, &posts)
	return posts
}

func parseRawPosts(posts []RawPost) (parsedPosts Posts) {
	for _, post := range posts {
		match := postTitlePattern.FindStringSubmatch(post.Title)
		if "" != match[1] && "" != match[2] {
			number, _ := strconv.Atoi(match[1])
			parsedPosts = append(parsedPosts, Post{
				Title:      post.Title,
				Url:        post.Url,
				Difficulty: match[2],
				Number:     number,
			})
		}

	}
	return
}
