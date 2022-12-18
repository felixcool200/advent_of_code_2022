use std::fs;

fn solve(input: &String, len: usize) -> usize {
    let mut char_buffer: Vec<char> = vec!['Q'; len];
    for (i, chr) in input.chars().enumerate() {
        //Insert in buffer
        for j in 0..char_buffer.len() - 1 {
            char_buffer[j] = char_buffer[j + 1];
        }
        char_buffer[len - 1] = chr;

        let mut new_buffer = char_buffer.clone();
        new_buffer.sort();
        let mut correct: bool = true;
        for j in 0..new_buffer.len() - 1 {
            if new_buffer[j] == 'Q' || new_buffer[j] == new_buffer[j + 1] {
                correct = false;
                break;
            }
        }
        if !correct {
            continue;
        }

        return i + 1; // 1 since it is indexed from 1 not 0
    }
    return input.len() + 1;
}

fn read_input(file_path: &str) -> String {
    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    return contents;
}

fn main() {
    let file_path: &str = "input.txt";
    let contents: String = read_input(file_path);
    println!("Part A:{}", solve(&contents, 4));
    println!("Part B:{}", solve(&contents, 14));
}
