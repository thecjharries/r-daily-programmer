// Copyright 2022 CJ Harries
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

// https://rocket.rs/v0.5-rc/guide/overview/#routing
#[macro_use]
extern crate rocket;

#[get("/ping")]
fn ping() -> &'static str {
    "pong"
}

#[cfg(not(tarpaulin_include))]
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![ping])
}

#[cfg(test)]
mod tests {
    use super::*;
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    #[test]
    fn test_ping() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let mut response = client.get(uri!(super::ping)).dispatch();
        assert_eq!(Status::Ok, response.status());
        assert_eq!("pong", response.into_string().unwrap());
    }
}
