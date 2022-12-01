use std::fs;

fn main() {
    // Read input from input.txt and split into one string per line
    let file_path = "input.txt";
    let all_input = fs::read_to_string(file_path)
        .expect("Input file should exist");
    let input_strings: Vec<&str> = all_input.split('\n').collect();

    // Build vector of total calories per elf
    let mut value = 0;
    let mut elf_values: Vec<i32> = Vec::new();

    for s in &input_strings {
        match s.parse::<i32>() {
            Ok(n) => value += n,
            Err(_) => { elf_values.push(value); value = 0},
        }
    }

    // Sort the elves, so one with most calories is last
    elf_values.sort();

    // Part 1 solution - elf with highst value
    println!("Elf with most has: {}", elf_values.last().unwrap());

    // Part 2 solution - sum the last 3 elves
    let mut total = 0;
    for i in elf_values.len()-3..elf_values.len() {
        total += elf_values[i];
    }
    println!("Top 3 have: {total}");

    // Alternative solution using sum of slice
    elf_values.reverse();
    println!("Top 3 have: {}", elf_values[0..3].iter().sum::<i32>());
}
