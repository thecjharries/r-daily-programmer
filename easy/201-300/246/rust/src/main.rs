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

struct Battery {
    voltage: f32,
    milliamp_hours: f32,
}

struct Led {
    voltage: f32,
    milliamp_hours: f32,
}

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn find_max_leds_for_runtime(hours: f32, led: &Led, battery: &Battery) -> f32 {
    0.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        let battery = Battery {
            voltage: 9.0,
            milliamp_hours: 1200.0,
        };
        let led = Led {
            voltage: 1.7,
            milliamp_hours: 20.0,
        };
        assert_eq!(300.0, find_max_leds_for_runtime(1.0, &led, &battery));
        assert_eq!(75.0, find_max_leds_for_runtime(4.0, &led, &battery));
        assert_eq!(35.0, find_max_leds_for_runtime(8.0, &led, &battery));
        assert_eq!(25.0, find_max_leds_for_runtime(12.0, &led, &battery));
    }
}
