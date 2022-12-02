use std::fs;
use map_macro::map;

fn score_rock(me: &str) -> i32 {
    match me {
        "X" => 3 + 1,
        "Y" => 6 + 2,
        "Z" => 0 + 3,
        _ => panic!("Invalid me input {me}"),
    }
}

fn score_paper(me: &str) -> i32 {
    match me {
        "X" => 0 + 1,
        "Y" => 3 + 2,
        "Z" => 6 + 3,
        _ => panic!("Invalid me input {me}"),
    }
}

fn score_scissors(me: &str) -> i32 {
    match me {
        "X" => 6 + 1,
        "Y" => 0 + 2,
        "Z" => 3 + 3,
        _ => panic!("Invalid me input {me}"),
    }
}

fn score_match(opponent: &str, me: &str) -> i32 {
    match opponent {
        "A" => score_rock(me),
        "B" => score_paper(me),
        "C" => score_scissors(me),
        _ => panic!("Invalid opponent input {opponent}"),
    }
}

fn score_rock2(me: &str) -> i32 {
    match me {
        "X" => 0 + 3,  // Lose - play scissors
        "Y" => 3 + 1,  // Draw - rock
        "Z" => 6 + 2,  // Win - paper
        _ => panic!("Invalid me input {me}"),
    }
}

fn score_paper2(me: &str) -> i32 {
    match me {
        "X" => 0 + 1,
        "Y" => 3 + 2,
        "Z" => 6 + 3,
        _ => panic!("Invalid me input {me}"),
    }
}

fn score_scissors2(me: &str) -> i32 {
    match me {
        "X" => 0 + 2,
        "Y" => 3 + 3,
        "Z" => 6 + 1,
        _ => panic!("Invalid me input {me}"),
    }
}

fn score_match2(opponent: &str, me: &str) -> i32 {
    match opponent {
        "A" => score_rock2(me),
        "B" => score_paper2(me),
        "C" => score_scissors2(me),
        _ => panic!("Invalid opponent input {opponent}"),
    }
}

fn main() {
    // Read input from input.txt and split into one string per line
    let file_path = "input.txt";
    let all_input = fs::read_to_string(file_path)
        .expect("Input file should exist");
    let input_strings: Vec<&str> = all_input.split('\n').collect();

    let mut total_score: i32 = 0;
    let mut total_score2: i32 = 0;
    for line in &input_strings {
        if line.len() == 0 { break; }
        let elements: Vec<&str> = line.split(' ').collect();
        total_score += score_match(elements[0], elements[1]);
        total_score2 += score_match2(elements[0], elements[1]);
    }

    println!("{total_score}, {total_score2}");

    // "One line" implementation - playing with map and fold to do a tuple sum
    let score1_map = map!{ "A X" => 3 + 1, "A Y" => 6 + 2, "A Z" => 0 + 3,
                           "B X" => 0 + 1, "B Y" => 3 + 2, "B Z" => 6 + 3,
                           "C X" => 6 + 1, "C Y" => 0 + 2, "C Z" => 3 + 3,
                           "" => 0 };
    let score2_map = map!{ "A X" => 0 + 3, "A Y" => 3 + 1, "A Z" => 6 + 2,
                           "B X" => 0 + 1, "B Y" => 3 + 2, "B Z" => 6 + 3,
                           "C X" => 0 + 2, "C Y" => 3 + 3, "C Z" => 6 + 1,
                           "" => 0 };

    (total_score, total_score2) = input_strings.iter().map(|&line| (score1_map[line], score2_map[line])).fold((0, 0), |acc, (x, y)| (acc.0 + x, acc.1 + y));
    
    println!("{total_score}, {total_score2}");
}
