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

#[derive(Debug)]
struct BitVec {
    bits: [bool; 8],
}

impl From<BitVec> for u8 {
    fn from(bitvec: BitVec) -> u8 {
        let mut result = 0;
        for (i, bit) in bitvec.bits.iter().enumerate() {
            if *bit {
                result |= 1 << i;
            }
        }
        result
    }
}

impl From<u8> for BitVec {
    fn from(value: u8) -> BitVec {
        let mut result = BitVec { bits: [false; 8] };
        for (i, bit) in result.bits.iter_mut().enumerate() {
            *bit = (value & (1 << i)) != 0;
        }
        result
    }
}

fn main() {
    let bitvector = BitVec {
        bits: [true, false, true, false, true, false, true, false],
    };

    let value: u8 = bitvector.into();

    println!("{}", value);
}
