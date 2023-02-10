// Copyright 2023 CJ Harries
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

use rocket::fs::NamedFile;
use rocket::{build, FromForm, get, launch, post, routes};
use rocket::serde::{Deserialize, Serialize};
use rocket::form::Form;
use std::io::Result;

#[derive(FromForm, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Data<'r> {
    text: &'r str,
}

#[get("/")]
async fn get_index() -> Result<NamedFile> {
    NamedFile::open("files/index.html").await
}

#[post("/", data = "<data>")]
async fn post_index(data: Form<Data<'_>>) -> String {
    format!("You sent: {}", data.text)
}

#[launch]
fn rocket() -> _ {
    build()
        .mount("/", routes![get_index, post_index])
}


#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;
    use rocket::local::blocking::{Client, LocalResponse};
    use std::fs::read_to_string;

    #[test]
    fn test_index() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response: LocalResponse = client.get("/").dispatch();
        assert_eq!(response.status(), rocket::http::Status::Ok);
        assert_eq!(
            response.content_type(),
            Some(rocket::http::ContentType::HTML)
        );
        assert_eq!(
            response.into_string().unwrap(),
            read_to_string("files/index.html").unwrap()
        );
    }
}
