use std::fs;
use std::collections::HashMap;

fn main() {
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let list_of_str = contents.split("\n");

    //Part A
    let mut score:u64 = 0; 
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
    
    for str_num in list_of_str{
        let read:Vec<&str> = str_num.split(" ").collect();
        if str_num == ""{
            continue;
        }
        //symbol score
        score += symbol_score[read[1]];

        //A draw
        if read[0] == symbol_same[read[1]]{
            score += 3;
            continue;
        }
        //A win
        if winner_map[read[0]] == read[1]{
            score += 6;
        }
        println!("{}",score);
    }
    //let list_of_str = contents.split("\n");
    println!("Score:{}",score);
    
}
