use std::fs;

fn get_range(section: &str) -> (i32, i32) {
    let mut it = section.split('-');
    let start = it.next().unwrap().parse::<i32>().unwrap();
    let end = it.next().unwrap().parse::<i32>().unwrap();

    (start, end)
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

    let mut total = 0;
    let mut total2 = 0;
    for line in &input_strings {
        let sections: Vec<&str> = line.split(',').collect();
        let elf1_range = get_range(sections[0]);
        let elf2_range = get_range(sections[1]);

        if (elf1_range.0 <= elf2_range.0 && elf1_range.1 >= elf2_range.1) ||
           (elf2_range.0 <= elf1_range.0 && elf2_range.1 >= elf1_range.1) {
               total += 1;
        }

        if (elf1_range.1 >= elf2_range.0 && elf1_range.0 <= elf2_range.1) {
            total2 += 1;
        }
    }

    println!("{total} {total2}");
}
