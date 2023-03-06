/*
Copyright 2022 Volker Schwaberow <volker@schwaberow.de>
Permission is hereby granted, free of charge, to any person obtaining a
copy of this software and associated documentation files (the
"Software"), to deal in the Software without restriction, including without
limitation the rights to use, copy, modify, merge, publish, distribute,
sublicense, and/or sell copies of the Software, and to permit persons to whom the
Software is furnished to do so, subject to the following conditions:
The above copyright notice and this permission notice shall be
included in all copies or substantial portions of the Software.
THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR
OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.
Author(s): Volker Schwaberow
*/

const MAX_FACTORIAL: usize = 120;

fn main() {
    let mut factorials = [1u64; MAX_FACTORIAL + 1];

    for i in 2..=MAX_FACTORIAL {
        factorials[i] = factorials[i - 1] * i as u64;
    }

    let choose = |n: u64, k: u64| -> u64 {
        factorials[n as usize] / factorials[k as usize] / factorials[(n - k) as usize]
    };

    for i in 1..12 {
        println!("P(12,{}) = {}", i, choose(12, i));
    }
    for i in 10..60 {
        println!("C(60,{}) = {}", i, choose(60, i));
    }
}
