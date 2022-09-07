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

use rust_fsm::*;

state_machine! {
    derive(Debug)
    GarageDoor(Closed)

    Closed => {
        ButtonClicked => Opening,
        CycleComplete => Closed,
    },
    Opening => {
        ButtonClicked => StoppedWhileOpening,
        CycleComplete => Open,
    },
    Open => {
        ButtonClicked => Closing,
        CycleComplete => Open,
    },
    Closing => {
        ButtonClicked => StoppedWhileClosing,
        CycleComplete => Closed,
    },
    StoppedWhileOpening => {
        ButtonClicked => Closing,
        CycleComplete => StoppedWhileOpening,
    },
    StoppedWhileClosing => {
        ButtonClicked => Opening,
        CycleComplete => StoppedWhileOpening,
    },
}

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn determine_final_state(inputs: Vec<&GarageDoorInput>) -> GarageDoorState {
    GarageDoorState::Closed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_determine_final_state() {
        let inputs = vec![
            &GarageDoorInput::ButtonClicked,
            &GarageDoorInput::CycleComplete,
            &GarageDoorInput::ButtonClicked,
            &GarageDoorInput::ButtonClicked,
            &GarageDoorInput::ButtonClicked,
            &GarageDoorInput::ButtonClicked,
            &GarageDoorInput::ButtonClicked,
            &GarageDoorInput::CycleComplete,
        ];
        assert_eq!(
            GarageDoorState::Opening,
            determine_final_state(inputs[..1].to_vec())
        );
        assert_eq!(
            GarageDoorState::Open,
            determine_final_state(inputs[..2].to_vec())
        );
        assert_eq!(GarageDoorState::Closed, determine_final_state(inputs));
    }
}
