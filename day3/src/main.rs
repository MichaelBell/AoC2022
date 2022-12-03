use std::fs;
use std::str;

fn get_priority(item: u8) -> i32 {
    if item >= b'a' && item <= b'z' {
        (item - b'a' + 1) as i32
    }
    else if item >= b'A' && item <= b'Z' {
        (item - b'A' + 27) as i32
    }
    else {
        panic!("Invalid item: {item}");
    }
}

fn find_common(a: &[u8], b: &[u8]) -> u8 {
    for c in a {
        if b.contains(c) {
            return *c;
        }
    }

    panic!("No common item between {} and {}", str::from_utf8(a).unwrap(), str::from_utf8(b).unwrap());
}

fn find_common_multi(v: &Vec<Vec<u8>>) -> u8 {
    for c in &v[0] {
        if v[1..v.len()].iter().all(|s| s.contains(&c)) {
            return *c;
        }
    }

    panic!("No common item");
}

fn split_input(all_input: &str) -> Vec<&str> {
    let input_strings: Vec<&str> = all_input.split('\n').collect();

    input_strings[0..input_strings.len()-1].to_vec()
}

fn main() {
    // Read input from input.txt and split into one string per line
    let file_path = "input.txt";
    let all_input = fs::read_to_string(file_path)
        .expect("Input file should exist");
    let input_strings = split_input(&all_input);

    let mut total: i32 = 0;
    for line in &input_strings {
        let line_bytes: Vec<u8> = line.bytes().collect();
        let line_len = line_bytes.len();
        let first = &line_bytes[0..line_len/2];
        let second = &line_bytes[line_len/2..line_len];

        let common = find_common(first, second);
        total += get_priority(common);
    }

    println!("{total}");

    total = 0;
    for i in (0..input_strings.len()).step_by(3) {
        // Not a masive fan of this - maybe I should be passing an iterator
        // through to get_common_multi?
        let lines_vec: Vec<Vec<u8>> = {
            let mut v = Vec::new();
            for line in &input_strings[i..i+3] {
                v.push(line.bytes().collect());
            }
            v
        };

        let common = find_common_multi(&lines_vec);
        total += get_priority(common);
    }
    println!("{total}");
}
