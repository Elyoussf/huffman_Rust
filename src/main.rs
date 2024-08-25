use std::collections::BinaryHeap;
use std::collections::HashMap;
use colored::*;

struct Tree {
    score: u64,
    label: Option<char>,  // Use Option<char> to handle leaf and non-leaf nodes
    left: Option<Box<Tree>>,
    right: Option<Box<Tree>>,
}

impl Ord for Tree {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.score.cmp(&self.score)
    }
}

impl PartialOrd for Tree {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Tree {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl Eq for Tree {}

fn frequencies(payload: &str) -> HashMap<char, u64> {
    let mut memo: HashMap<char, u64> = HashMap::new();
    for chara in payload.chars() {
        *memo.entry(chara).or_insert(0) += 1;
    }
    memo
}

fn main() {
    let text = "Huffman coding is a lossless data compression algorithm...";
    let freq = frequencies(text);
    let mut min_heap: BinaryHeap<Tree> = BinaryHeap::new();

    for (&ch, &freq) in &freq {
        let current_tree = Tree {
            score: freq,
            label: Some(ch),
            left: None,
            right: None,
        };
        min_heap.push(current_tree);
    }

    while min_heap.len() >= 2 {
        let node1 = min_heap.pop().unwrap();
        let node2 = min_heap.pop().unwrap();
        let new_node = Tree {
            score: node1.score + node2.score,
            label: None,  // Non-leaf nodes don't need a label
            left: Some(Box::new(node1)),
            right: Some(Box::new(node2)),
        };
        min_heap.push(new_node);
    }

    let head = min_heap.pop().unwrap();
    let mut codes: HashMap<String, String> = HashMap::new();
    find_character(&head, &mut codes, String::new());

    let mut size_original = 0;
    let mut size_compressed = 0;

    for (label, code) in codes {
        let ch = label.chars().next().unwrap();
        size_compressed += (code.len() as u64) * freq[&ch];
        size_original += freq[&ch] * 8;
    }

    println!("The original size is {} bits", size_original.to_string().red());
    println!("The size of the compressed one is {} bits", size_compressed.to_string().green());
}

fn find_character(head: &Tree, codes: &mut HashMap<String, String>, current_code: String) {
    if head.left.is_none() && head.right.is_none() {
        if let Some(label) = head.label {
            codes.insert(label.to_string(), current_code);
        }
        return;
    }

    if let Some(left) = &head.left {
        find_character(left, codes, current_code.clone() + "0");
    }
    if let Some(right) = &head.right {
        find_character(right, codes, current_code + "1");
    }
}

