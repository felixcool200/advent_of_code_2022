use std::fs;
use std::str::Split;

//Part A
fn part_a(list_of_str: Split<&str>) -> String {
    //Read first state
    return solve(list_of_str, true);
}

//Part B
fn part_b(list_of_str: Split<&str>) -> String {
    return solve(list_of_str, false);
}

fn solve(list_of_str: Split<&str>, is_a: bool) -> String {
    //Read first state
    let mut stacks: Vec<Vec<char>> = Vec::new();
    let mut state: bool = true;
    for str_num in list_of_str {
        if state {
            //Fix new line in input from VIM
            if str_num == "" {
                state = false;
                continue;
            }
            let mut tmp: Vec<char> = Vec::new();
            for chr in str_num.chars() {
                tmp.push(chr);
            }
            stacks.push(tmp);
        } else {
            if str_num == "" {
                continue;
            }
            let command: Vec<&str> = str_num.split(" ").collect();
            let mut cmd: Vec<usize> = Vec::new();
            for cm in (1..command.len()).step_by(2) {
                cmd.push(command[cm].parse::<usize>().unwrap());
            }
            match is_a {
                true => a_move(&mut stacks, &cmd),
                false => b_move(&mut stacks, &cmd),
            };
        }
    }
    let mut answer: String = String::new();
    for mut stack in stacks {
        answer.push(stack.pop().unwrap());
    }
    return answer;
}

fn a_move(stacks: &mut Vec<Vec<char>>, cmd: &Vec<usize>) {
    //println!("Moving: {} from {} to {}", cmd[0], cmd[1], cmd[2]);
    for _ in 0..cmd[0] {
        let value: char = stacks[cmd[1] - 1].pop().unwrap();
        //println!("Moved Now: {}", value);
        stacks[cmd[2] - 1].push(value);
    }
}

fn b_move(stacks: &mut Vec<Vec<char>>, cmd: &Vec<usize>) {
    let mut tmp: Vec<char> = Vec::new();
    for _ in 0..cmd[0] {
        let value: char = stacks[cmd[1] - 1].pop().unwrap();
        //println!("Moved Now: {}", value);
        tmp.push(value);
    }
    tmp.reverse();
    stacks[cmd[2] - 1].append(&mut tmp);
}

fn read_input(file_path: &str) -> String {
    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    return contents;
}

fn main() {
    let file_path: &str = "input.txt";
    let contents: String = read_input(file_path);
    let list_of_str: Split<&str> = contents.split("\n");

    println!("Part A:{}", part_a(list_of_str.clone()));
    println!("Part B:{}", part_b(list_of_str.clone()));
}
