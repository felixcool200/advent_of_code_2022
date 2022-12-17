use std::fs;
use std::collections::HashMap;
use std::str::Split;

//Part A
fn part_a(list_of_str:Split<&str>, priority_map:&HashMap<char,usize>) -> usize{
    let mut sum:usize = 0;  
    for str_num in list_of_str{
        //Fix new line in input from VIM
        if str_num == ""{
            continue;
        }
        let len:usize = str_num.len()/2;
        sum += match find_matching_letters(&str_num[..len],&str_num[len..]){
            Ok(letter) => priority_map[&letter],
            Err(err_msg) => {
                println!("ERROR: {}",err_msg);
                0
            },
        }
    }  
    return sum;
}

fn find_matching_letters(str1: &str,str2: &str) -> Result<char,&'static str>{
    for letter in str1.chars(){
        for letter2 in str2.chars(){
            if letter == letter2{
                return Ok(letter);
            }
        }
    }
    return Err("Could not find match");
}

//Part B
fn part_b(list_of_str:Split<&str>, priority_map:&HashMap<char,usize>) -> usize{
    let mut sum:usize = 0;  
    let mut bags: Vec<&str> = Vec::new();
    for str_num in list_of_str{
        if str_num == ""{ //Igonre empty lines (VIM adds one to the end)
            continue;
        }
        bags.push(str_num);
        if bags.len() == 3{
            sum += match find_badges(&bags){
                Ok(letter) => priority_map[&letter],
                Err(err_msg) => { 
                    println!("ERROR: {}",err_msg);
                    0
                }
            };
            bags.clear();
        }
    }
    return sum;
    }

fn find_badges(bags: &Vec<&str>) -> Result<char,&'static str>{
    for letter in bags[0].chars(){
        for letter2 in bags[1].chars(){
            if letter == letter2{
                for letter3 in bags[2].chars(){
                    if letter == letter3{
                        return Ok(letter);
                    }
                }
            }
        }
    }
    return Err("No letter found");
}

fn read_input(file_path: &str) -> String{
    let contents:String = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    return contents
}

fn create_hash_map() -> HashMap<char,usize>{
    let letters: &str = "abcdefghijklmnopqrstuvwxyz";
    let capital_letters: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut priority_map: HashMap<char,usize> = HashMap::new();
    for (i,letter) in letters.chars().enumerate(){
        priority_map.insert(letter,i+1);
    }
    for (i,letter ) in capital_letters.chars().enumerate(){
        priority_map.insert(letter,i+1+26);
    }
    return priority_map;
}

fn main() {
    let file_path:&str = "input.txt";
    let contents:String = read_input(file_path);
    let priority_map:HashMap<char,usize> = create_hash_map();
    let list_of_str:Split<&str> = contents.split("\n");

    println!("Part A:{}",part_a(list_of_str.clone(),&priority_map));
    println!("Part B:{}",part_b(list_of_str.clone(),&priority_map));

}

