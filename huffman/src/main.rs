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

use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Debug, Eq)]
struct Node {
    character: Option<char>,
    frequency: u64,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(character: Option<char>, frequency: u64) -> Node {
        Node {
            character,
            frequency,
            left: None,
            right: None,
        }
    }
}

#[derive(Eq)]
struct Wrapper(Node);

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.character == other.character
            && self.frequency == other.frequency
            && self.left == other.left
            && self.right == other.right
    }
}

impl Ord for Wrapper {
    fn cmp(&self, other: &Wrapper) -> Ordering {
        other.0.frequency.cmp(&self.0.frequency)
    }
}

impl PartialOrd for Wrapper {
    fn partial_cmp(&self, other: &Wrapper) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Wrapper {
    fn eq(&self, other: &Wrapper) -> bool {
        self.0.frequency == other.0.frequency
    }
}

fn build_tree(frequency_map: &[(char, u64)]) -> Node {
    let mut heap = BinaryHeap::new();

    for &(c, f) in frequency_map {
        heap.push(Wrapper(Node::new(Some(c), f)));
    }

    while heap.len() > 1 {
        let wrapper1 = heap.pop().unwrap();
        let wrapper2 = heap.pop().unwrap();

        let node1 = wrapper1.0;
        let node2 = wrapper2.0;

        let mut merged = Node::new(None, node1.frequency + node2.frequency);
        merged.left = Some(Box::new(node1));
        merged.right = Some(Box::new(node2));

        heap.push(Wrapper(merged));
    }

    heap.pop().unwrap().0
}

fn main() {
    let frequency_map = [('v', 1), ('o', 1), ('l', 2), ('k', 1), ('e', 1), ('r', 1)];
    let tree = build_tree(&frequency_map);

    let mut encoding = String::new();
    let mut current_code = String::new();
    create_encoding(&tree, &mut encoding, &mut current_code);

    println!("Encoding: {:?}", encoding);
}

fn create_encoding(node: &Node, encoding: &mut String, current_code: &mut String) {
    if let Some(c) = node.character {
        encoding.push_str(current_code);
        encoding.push(c);
    } else {
        current_code.push('0');
        create_encoding(node.left.as_ref().unwrap(), encoding, current_code);
        current_code.pop();

        current_code.push('1');
        create_encoding(node.right.as_ref().unwrap(), encoding, current_code);
        current_code.pop();
    }
}
