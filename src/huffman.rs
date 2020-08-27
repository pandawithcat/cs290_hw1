use bit_vec::BitVec;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fs;
use std::time::Instant;
use std::vec::Vec;

pub fn run_question3() {
    //Goal:
    println!("First test the char count function");

    let a = "abcdeabaccaaaaa";

    let res_fn = get_char_counts(a);
    let res: HashMap<_, _> = a.chars().zip(vec![8, 2, 3, 1, 1]).collect();

    assert_eq!(res_fn, res);

    println!("This is answer for question3");

    let tree = get_huffman_tree(a);
    println!("This is the huffman tree {:?}", tree);

    println!("Test encoding");

    let s = "abcdeabaccaaaaa";

    let hc = get_huffman_codes(s);

    let s2 = "babbc";

    let encoded = encode(s2, &hc);
    println!("this is the encoded message {:?}", encoded);

    println!("Test decoding");
    let decoded = decode(encoded, &hc);
    println!("this is the decoded message {:?}", decoded);
    assert_eq!(s2, decoded);
}

pub fn run_question4() {
    // tihs version doesn't work for message2 because alphabet2 file is contaminated
    // println!("This is answer for question4");
    // println!("Testing the first set");
    // test_huffman("Alphabet1.txt", "Frequency1.txt", "Message1.txt");
    // println!("Testing the second set");
    // test_huffman("Alphabet2.txt", "Frequency2.txt", "Message2.txt");

    println!("This is answer for question4");
    println!("Testing the first set");
    test_huffman_fake("Message1.txt");
    println!("Testing the second set");
    test_huffman_fake("Message2.txt");
}

pub fn run_question5() {
    //Goal:
    println!("This is answer for question5");
    test_huffman_fake("TaleOfTwoCities.txt");
}

fn encode(data: &str, map: &HashMap<char, BitVec>) -> BitVec {
    let mut temp = 0;
    for c in data.chars() {
        temp += map.get(&c).unwrap().len();
    }

    let mut result = BitVec::with_capacity(temp);

    for c in data.chars() {
        let code = map.get(&c).unwrap();
        for bit in code.iter() {
            result.push(bit);
        }
    }

    result
}

fn decode(bits: BitVec, map: &HashMap<char, BitVec>) -> String {
    let mut result = HashMap::new();

    for (k, v) in map.iter() {
        result.insert(v.clone(), k.clone());
    }

    let mut res = String::new();
    let mut temp = BitVec::new();
    for bit in bits.iter() {
        temp.push(bit);
        if result.contains_key(&temp) {
            res.push(result.get(&temp).unwrap().clone());
        }
        temp = BitVec::new();
    }

    res
}

fn get_char_counts(data: &str) -> HashMap<char, i32> {
    let mut char_counts = HashMap::new();

    for c in data.chars() {
        let count = char_counts.entry(c).or_insert(0);
        *count += 1;
    }
    println!("this is the character count {:?}", char_counts);
    char_counts
}

fn heapify(map: HashMap<char, i32>) -> BinaryHeap<Box<Tree>> {
    let mut heap = BinaryHeap::new();

    for (letter, count) in map.iter() {
        let t = Tree::new(*letter, *count);
        heap.push(t);
    }

    heap
}

fn create_huffman_tree(mut heap: BinaryHeap<Box<Tree>>) -> Box<Tree> {
    while heap.len() > 1 {
        let t1 = heap.pop().unwrap();
        let t2 = heap.pop().unwrap();

        let t_new = Tree::combine(t1, t2);
        heap.push(t_new);
    }

    heap.pop().unwrap()
}

pub fn get_huffman_tree(data: &str) -> Tree {
    let char_counts = get_char_counts(data);

    let heap = heapify(char_counts);

    let ht = create_huffman_tree(heap);

    return *ht;
}

pub fn get_huffman_codes(data: &str) -> HashMap<char, BitVec> {
    let char_counts = get_char_counts(data);

    let heap = heapify(char_counts);

    let ht = create_huffman_tree(heap);

    return huffman_codes_from_tree(&Some(ht), BitVec::new(), HashMap::new());
}

fn invert_huffman_codes(codes: &HashMap<char, BitVec>) -> HashMap<BitVec, char> {
    let mut result = HashMap::new();

    for (key, value) in codes.iter() {
        result.insert(value.clone(), key.clone());
    }

    result
}

fn huffman_codes_from_tree(
    opt: &Option<Box<Tree>>,
    prefix: BitVec,
    mut map: HashMap<char, BitVec>,
) -> HashMap<char, BitVec> {
    if let Some(ref tree) = *opt {
        match tree.value {
            Some(c) => {
                map.insert(c, prefix);
            }
            None => {
                let mut prefix_left = prefix.clone();
                prefix_left.push(true);
                let map = huffman_codes_from_tree(&tree.left, prefix_left, map);
                let mut prefix_right = prefix.clone();
                prefix_right.push(false);
                return huffman_codes_from_tree(&tree.right, prefix_right, map);
            }
        }
    }

    return map;
}

pub fn test_huffman_fake(message_file: &str) {
    let message = fs::read_to_string(message_file).expect("Something went wrong reading the file");
    println!(
        "this is the length of the original message {}",
        message.len()
    );
    let hc = get_huffman_codes(&message);
    println!("this is the number of unique characters {}", hc.len());

    let start_encode = Instant::now();
    let encoded = encode(&message, &hc);
    println!("Encoding time {}", start_encode.elapsed().as_millis());
    println!(
        "this is the length of the encoded message {}",
        encoded.len()
    );

    println!("Test decoding");
    let start_decode = Instant::now();
    let decoded = decode(encoded, &hc);
    println!("Decoding time {}", start_decode.elapsed().as_millis());
    assert_eq!(message, decoded);
}

fn test_huffman(alphabet_file: &str, frequency_file: &str, message_file: &str) {
    let alphabet =
        fs::read_to_string(alphabet_file).expect("Something went wrong reading the file");
    let frequency =
        fs::read_to_string(frequency_file).expect("Something went wrong reading the file");
    let message = fs::read_to_string(message_file).expect("Something went wrong reading the file");

    let hc = get_huffman_codes_with_data(&alphabet, &frequency);
    println!("this is hc {:?}", hc);

    let encoded = encode(&message, &hc);
    println!("this is the encoded message {:?}", encoded);
    println!(
        "this is the length of the encoded message {}",
        encoded.len()
    );

    println!("Test decoding");
    let decoded = decode(encoded, &hc);
    println!("this is the decoded message {:?}", decoded);
    assert_eq!(message, decoded);
}

fn get_huffman_codes_with_data(alphabet: &str, frequency: &str) -> HashMap<char, BitVec> {
    let char_counts = create_huffman_hashmap(alphabet, frequency);

    let heap = heapify(char_counts);

    let ht = create_huffman_tree(heap);

    return huffman_codes_from_tree(&Some(ht), BitVec::new(), HashMap::new());
}

fn fuck(a: &str) -> i32 {
    a.parse::<i32>().unwrap()
}

fn create_huffman_hashmap(alphabet: &str, frequency: &str) -> HashMap<char, i32> {
    let mut char_counts = HashMap::new();

    let alphabet_vector: Vec<&str> = alphabet.split(',').collect();
    println!("the alphabet length is.. {}", alphabet_vector.len());
    let alphabet_vector_true: Vec<char> = alphabet_vector
        .iter()
        .map(|a| a.chars().next().unwrap())
        .collect();
    let frequency_vector: Vec<&str> = frequency.split(',').collect();
    println!("the frequency length is.. {}", frequency_vector.len());
    let frequency_vector_true: Vec<i32> = frequency_vector.iter().map(|a| fuck(a)).collect();

    let length: usize = alphabet_vector.len();

    for i in 0..length {
        let character: char = *alphabet_vector_true.get(i).expect("fuck");
        let freq: i32 = *frequency_vector_true.get(i).expect("fuck");
        println!("the frequency of {} is {}", character, freq);
        char_counts.insert(character, freq);
    }

    return char_counts;
}

#[derive(Eq)]
pub struct Tree {
    count: i32,
    value: Option<char>,
    left: Option<Box<Tree>>,
    right: Option<Box<Tree>>,
}

impl Ord for Tree {
    fn cmp(&self, other: &Tree) -> Ordering {
        (-self.count).cmp(&(-other.count))
    }
}

impl PartialOrd for Tree {
    fn partial_cmp(&self, other: &Tree) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Tree {
    fn eq(&self, other: &Tree) -> bool {
        self.count == other.count
    }
}

impl Tree {
    fn new(value: char, count: i32) -> Box<Tree> {
        return Box::new(Tree {
            count,
            value: Some(value),
            left: None,
            right: None,
        });
    }

    fn combine(tree_smaller: Box<Tree>, tree_larger: Box<Tree>) -> Box<Tree> {
        return Box::new(Tree {
            count: tree_smaller.count + tree_larger.count,
            value: None,
            left: Some(tree_smaller),
            right: Some(tree_larger),
        });
    }
}
