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

fn main() {
    println!("rad");
}

fn hexdump(file_path: &str) -> Vec<String> {
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hexdump() {
        assert_eq!(
            hexdump("./Makefile"),
            "00000000: 23 20 41 6c 69 61 73 65 73 20 66 6f 72 20 65 78  |# Aliases for ex|
00000010: 65 63 75 74 61 62 6c 65 73 0a 43 41 52 47 4f 20  |ecutables.CARGO |
00000020: 3a 3d 20 63 61 72 67 6f 0a 47 49 54 20 3a 3d 20  |:= cargo.GIT := |
00000030: 67 69 74 0a 53 45 44 20 3a 3d 20 73 65 64 0a 0a  |git.SED := sed..|
00000040: 23 20 56 61 72 69 61 62 6c 65 73 0a 44 49 46 46  |# Variables.DIFF|
00000050: 49 43 55 4c 54 59 20 3d 20 24 28 6c 61 73 74 77  |ICULTY = $(lastw|
00000060: 6f 72 64 20 24 28 73 75 62 73 74 20 2f 2c 20 2c  |ord $(subst /, ,|
00000070: 24 28 72 65 61 6c 70 61 74 68 20 2e 2e 2f 2e 2e  |$(realpath ../..|
00000080: 2f 2e 2e 2f 29 29 29 0a 4e 55 4d 42 45 52 20 3d  |/../))).NUMBER =|
00000090: 20 24 28 6c 61 73 74 77 6f 72 64 20 24 28 73 75  | $(lastword $(su|
000000a0: 62 73 74 20 2f 2c 20 2c 24 28 72 65 61 6c 70 61  |bst /, ,$(realpa|
000000b0: 74 68 20 2e 2e 2f 29 29 29 0a 0a 23 20 52 75 6e  |th ../)))..# Run|
000000c0: 20 74 68 65 20 70 72 6f 67 72 61 6d 0a 72 75 6e  | the program.run|
000000d0: 3a 0a 09 24 28 43 41 52 47 4f 29 20 72 75 6e 0a  |:..$(CARGO) run.|
000000e0: 0a 23 20 52 75 6e 20 74 68 65 20 74 65 73 74 73  |.# Run the tests|
000000f0: 0a 74 65 73 74 3a 0a 09 24 28 43 41 52 47 4f 29  |.test:..$(CARGO)|
00000100: 20 74 65 73 74 0a 0a 23 20 53 65 74 20 75 70 20  | test..# Set up |
00000110: 74 68 65 20 64 61 69 6c 79 20 65 78 65 72 63 69  |the daily exerci|
00000120: 73 65 0a 62 6f 6f 74 73 74 72 61 70 2d 66 65 61  |se.bootstrap-fea|
00000130: 74 75 72 65 2d 62 72 61 6e 63 68 3a 0a 09 24 28  |ture-branch:..$(|
00000140: 47 49 54 29 20 66 6c 6f 77 20 66 65 61 74 75 72  |GIT) flow featur|
00000150: 65 20 73 74 61 72 74 20 24 28 44 49 46 46 49 43  |e start $(DIFFIC|
00000160: 55 4c 54 59 29 2d 24 28 4e 55 4d 42 45 52 29 0a  |ULTY)-$(NUMBER).|
00000170: 09 24 28 47 49 54 29 20 61 64 64 20 2e 2f 4d 61  |.$(GIT) add ./Ma|
00000180: 6b 65 66 69 6c 65 0a 09 24 28 47 49 54 29 20 63  |kefile..$(GIT) c|
00000190: 6f 6d 6d 69 74 20 2d 6d 20 27 41 64 64 20 62 6f  |ommit -m 'Add bo|
000001a0: 69 6c 65 72 70 6c 61 74 65 27 0a 09 24 28 47 49  |ilerplate'..$(GI|
000001b0: 54 29 20 61 64 64 20 2e 2f 73 72 63 2f 6d 61 69  |T) add ./src/mai|
000001c0: 6e 2e 72 73 0a 09 24 28 47 49 54 29 20 63 6f 6d  |n.rs..$(GIT) com|
000001d0: 6d 69 74 20 2d 6d 20 27 43 72 65 61 74 65 20 65  |mit -m 'Create e|
000001e0: 6d 70 74 79 20 52 75 73 74 20 66 69 6c 65 27 0a  |mpty Rust file'.|
000001f0: 09 24 28 53 45 44 29 20 2d 69 20 27 73 2f 26 52  |.$(SED) -i 's/&R|
00000200: 45 50 4c 41 43 45 5f 4d 45 26 2f 24 28 44 49 46  |EPLACE_ME&/$(DIF|
00000210: 46 49 43 55 4c 54 59 29 5f 24 28 4e 55 4d 42 45  |FICULTY)_$(NUMBE|
00000220: 52 29 2f 67 27 20 2e 2f 43 61 72 67 6f 2e 74 6f  |R)/g' ./Cargo.to|
00000230: 6d 6c 0a 09 24 28 47 49 54 29 20 61 64 64 20 2e  |ml..$(GIT) add .|
00000240: 2f 43 61 72 67 6f 2e 74 6f 6d 6c 0a 09 24 28 47  |/Cargo.toml..$(G|
00000250: 49 54 29 20 63 6f 6d 6d 69 74 20 2d 6d 20 27 44  |IT) commit -m 'D|
00000260: 65 66 69 6e 65 20 52 75 73 74 20 70 61 63 6b 61  |efine Rust packa|
00000270: 67 65 27 0a 0a 23 20 53 68 6f 72 74 63 75 74 20  |ge'..# Shortcut |
00000280: 74 6f 20 66 69 6e 69 73 68 20 74 68 65 20 65 78  |to finish the ex|
00000290: 65 72 63 69 73 65 20 62 72 61 6e 63 68 0a 23 20  |ercise branch.# |
000002a0: 50 72 6f 62 61 62 6c 79 20 62 65 74 74 65 72 20  |Probably better |
000002b0: 61 73 20 61 6e 20 61 6c 69 61 73 3b 20 74 68 69  |as an alias; thi|
000002c0: 73 20 77 6f 72 6b 73 20 6a 75 73 74 20 66 69 6e  |s works just fin|
000002d0: 65 0a 66 69 6e 69 73 68 3a 20 74 65 73 74 0a 09  |e.finish: test..|
000002e0: 24 28 47 49 54 29 20 66 6c 6f 77 20 66 65 61 74  |$(GIT) flow feat|
000002f0: 75 72 65 20 66 69 6e 69 73 68 0a 0a 23 20 43 6f  |ure finish..# Co|
00000300: 6e 76 65 6e 69 65 6e 63 65 20 74 61 72 67 65 74  |nvenience target|
00000310: 20 66 6f 72 20 63 6f 6d 6d 6f 6e 20 63 6f 6d 6d  | for common comm|
00000320: 69 74 0a 23 20 43 6f 6d 6d 69 74 73 20 61 20 70  |it.# Commits a p|
00000330: 72 69 6d 61 72 79 20 73 74 75 62 20 66 6f 72 20  |rimary stub for |
00000340: 74 65 73 74 69 6e 67 0a 73 74 75 62 3a 0a 09 24  |testing.stub:..$|
00000350: 28 47 49 54 29 20 63 6f 6d 6d 69 74 20 73 72 63  |(GIT) commit src|
00000360: 2f 6d 61 69 6e 2e 72 73 20 2d 6d 20 27 53 74 75  |/main.rs -m 'Stu|
00000370: 62 20 70 72 6f 6d 70 74 20 66 6e 63 27 0a 0a 23  |b prompt fnc'..#|
00000380: 20 43 6f 6e 76 65 6e 69 65 6e 63 65 20 74 61 72  | Convenience tar|
00000390: 67 65 74 20 66 6f 72 20 63 6f 6d 6d 6f 6e 20 63  |get for common c|
000003a0: 6f 6d 6d 69 74 0a 23 20 43 6f 6d 6d 69 74 73 20  |ommit.# Commits |
000003b0: 6d 61 69 6e 20 6d 65 74 68 6f 64 20 75 70 64 61  |main method upda|
000003c0: 74 65 64 20 77 69 74 68 20 70 72 6f 6d 70 74 20  |ted with prompt |
000003d0: 74 61 73 6b 0a 67 69 74 2d 70 72 6f 6d 70 74 2d  |task.git-prompt-|
000003e0: 72 75 6e 6e 65 72 3a 0a 09 24 28 47 49 54 29 20  |runner:..$(GIT) |
000003f0: 61 64 64 20 73 72 63 2f 6d 61 69 6e 2e 72 73 0a  |add src/main.rs.|
00000400: 09 24 28 47 49 54 29 20 63 6f 6d 6d 69 74 20 2d  |.$(GIT) commit -|
00000410: 6d 20 27 41 64 64 20 70 72 6f 6d 70 74 20 72 75  |m 'Add prompt ru|
00000420: 6e 6e 65 72 20 74 6f 20 6d 61 69 6e 27 0a 0a 70  |nner to main'..p|
00000430: 61 74 63 68 3a 0a 09 24 28 47 49 54 29 20 61 64  |atch:..$(GIT) ad|
00000440: 64 20 73 72 63 20 2d 2d 70 61 74 63 68 0a        |d src --patch.|
"
        );
    }
}
