use std::fs;
use std::collections::HashMap;

fn input_to_symbol_part_a(inp_1:&str, inp_2:&str) -> usize {

    let symbol_same = HashMap::from([
        ("X","A"),
        ("Y","B"),
        ("Z","C"),
    ]);

    let symbol_score = HashMap::from([
        ("X",1),
        ("Y",2),
        ("Z",3),
    ]);

    let winner_map = HashMap::from([
        ("A", "Y"), // Paper beats rock
        ("B", "Z"), // Scissor beats paper
        ("C", "X"), // Rock beats scissor
    ]);
    //symbol score
    let score = symbol_score[inp_2];

    //A draw
    if inp_1 == symbol_same[inp_2]{
        return score + 3;
    }
    
    //A win
    if winner_map[inp_1] == inp_2{
        return score + 6;
    }
    // A loss
    return score;
}

fn input_to_symbol_part_b(inp_1:&str, inp_2:&str) -> usize {
    //The three HashMaps are offset by a integer thus a list with offset could be used
    let win_map = HashMap::from([
        ("A","Y"),
        ("B","Z"),
        ("C","X"),
    ]);
    let lose_map = HashMap::from([
        ("A","Z"),
        ("B","X"),
        ("C","Y"),
    ]);
    let same_map = HashMap::from([
        ("A","X"),
        ("B","Y"),
        ("C","Z"),
    ]);
    let mut imp:&str = "";
    //Remap to part_a syntax
    if inp_2 == "X"{ // I should lose
        imp = lose_map[inp_1];
    }
    if inp_2 == "Y"{ // I should draw
        imp = same_map[inp_1];
    }
    if inp_2 == "Z"{ // I should win
        imp = win_map[inp_1];
    }
    return input_to_symbol_part_a(inp_1,imp)
}

fn main() {
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let list_of_str = contents.split("\n");

    //Part A
    let mut score_part_a:usize = 0;  
    let mut score_part_b:usize = 0;  
    for str_num in list_of_str{
        let input_vec:Vec<&str> = str_num.split(" ").collect();
        if str_num == ""{
            continue;
        }
        score_part_a += input_to_symbol_part_a(&input_vec[0],&input_vec[1]);
        score_part_b += input_to_symbol_part_b(&input_vec[0],&input_vec[1]);
        //println!("{}",score);
    }
    //let list_of_str = contents.split("\n");
    println!("Part A:{}",score_part_a);
    println!("Part B:{}",score_part_b);

}

