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

use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashSet;

fn get_characters(word: &str) -> Vec<char> {
    word.chars().collect()
}

fn shuffle_characters(characters: &mut [char]) {
    let mut rng = thread_rng();
    characters.shuffle(&mut rng);
}

fn build_anagram(word: &str) -> String {
    let mut characters = get_characters(word);
    shuffle_characters(&mut characters);
    characters.into_iter().collect()
}

fn generate_all_anagrams(word: &str) -> Vec<String> {
    let mut anagrams = Vec::new();
    let mut seen_anagrams = HashSet::new();
    while seen_anagrams.len() < factorial(word.len()) {
        let anagram = build_anagram(word);
        if !seen_anagrams.contains(&anagram) {
            seen_anagrams.insert(anagram.clone());
            anagrams.push(anagram);
        }
    }
    anagrams
}

fn factorial(n: usize) -> usize {
    (1..=n).product()
}

fn main() {
    let word = match std::env::args().nth(1) {
        Some(arg) => arg,
        None => {
            eprintln!("Usage: {} <word>", std::env::args().nth(0).unwrap());
            std::process::exit(1);
        }
    };
    let anagrams = generate_all_anagrams(&word);
    anagrams.iter().for_each(|anagram| println!("{}", anagram));
}
