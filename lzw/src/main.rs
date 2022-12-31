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

use std::{collections::HashMap, error::Error};

fn compress(data: &[u8]) -> Result<Vec<u32>, Box<dyn Error>> {
    let mut dictionary: HashMap<Vec<u8>, u32> = (0u32..=255).map(|i| (vec![i as u8], i)).collect();

    let mut w = Vec::new();
    let mut compressed = Vec::new();

    for &b in data {
        let mut wc = w.clone();
        wc.push(b);

        match dictionary.contains_key(&wc) {
            true => w = wc,
            false => {
                compressed.push(dictionary[&w]);
                dictionary.insert(wc, dictionary.len() as u32);
                w.clear();
                w.push(b);
            }
        }
    }

    if !w.is_empty() {
        compressed.push(dictionary[&w]);
    }

    Ok(compressed)
}

fn decompress(mut data: &[u32]) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut dictionary: HashMap<u32, Vec<u8>> = (0u32..=255).map(|i| (i, vec![i as u8])).collect();

    let mut w = dictionary[&data[0]].clone();
    data = &data[1..];
    let mut decompressed = w.clone();

    for &k in data {
        let entry = match (dictionary.contains_key(&k), k) {
            (true, _) => dictionary[&k].clone(),
            (false, n) if n == dictionary.len() as u32 => {
                let mut entry = w.clone();
                entry.push(w[0]);
                entry
            }
            _ => {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Invalid dictionary!",
                )))
            }
        };

        decompressed.extend_from_slice(&entry);

        w.push(entry[0]);
        dictionary.insert(dictionary.len() as u32, w);

        w = entry;
    }

    Ok(decompressed)
}

fn main() -> Result<(), Box<dyn Error>> {
    let compressed = match compress("Volker Schwaberow".as_bytes()) {
        Ok(v) => v,
        Err(e) => return Err(e),
    };

    println!("{:?}", compressed);

    let decompressed = decompress(&compressed)?;
    let decompressed = String::from_utf8(decompressed)?;
    println!("{}", decompressed);

    Ok(())
}
