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

// use std::num::sqrt;

fn main() {
    println!("{}", chudnovsky(1000000));
}

// https://www.craig-wood.com/nick/articles/pi-chudnovsky/
fn chudnovsky(n: f64) -> f64 {
    let mut k: f64 = 1;
    let mut a_k: f64 = n;
    let mut a_sum: f64 = n;
    let mut b_sum: f64 = 0;
    const C: f64 = 640320;
    let C3_OVER_24: f64 = C.pow(3) / 24;
    println!("{}", C3_OVER_24);
    while 0 != a_k  {
        a_k *= -1*(6*k-5)*(2*k-1)*(6*k-1);
        println!("{} {} {}", a_k, a_sum, b_sum);
        a_k /= k*k*k*C3_OVER_24;
        a_sum += a_k;
        b_sum += k * a_k;
        k += 1;
    }
    let total: f64 = 13591409*a_sum + 545140134*b_sum;
    let pi: f64 = 426880*((10005.0 as f64).sqrt() as f64)/total;
    pi
}
