use std::fs;
use std::collections::HashMap;

fn find_priority(bags: &Vec<&str>,priority_map: &HashMap<char,usize>) -> usize{
    for letter in bags[0].chars(){
        for letter2 in bags[1].chars(){
            for letter3 in bags[2].chars(){
                if letter == letter2 && letter == letter3{
                    let priority = priority_map[&letter];
                    println!("letter:{}, priority: {}",letter,priority);
                    return priority;
                }
            }
        }
    }
    return 0;
}

fn main() {

    let letters: &str = "abcdefghijklmnopqrstuvwxyz";
    let capital_letters: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut priority_map: HashMap<char,usize> = HashMap::new();
    for (i,letter) in letters.chars().enumerate(){
        priority_map.insert(letter,i+1);
    }
    for (i,letter) in capital_letters.chars().enumerate(){
        priority_map.insert(letter,i+1+26);
    }

    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let list_of_str = contents.split("\n");

    //Part A
    let mut sum:usize = 0;  
    for str_num in list_of_str{
        let len:usize = str_num.len()/2;
        let bag1: &str = &str_num[..len];
        let bag2: &str = &str_num[len..];
        for letter in bag1.chars(){
            let last_sum:usize = sum;
            for letter2 in bag2.chars(){
                if letter == letter2{
                    let priority = priority_map[&letter];
                    //println!("letter:{}, priority: {}",letter,priority);
                    sum += priority;
                    break;
                }
            }
            if last_sum != sum{
                break;
            }
        }
        //Fix new line in input from VIM
        if str_num == ""{
            continue;
        }
    }
    //let list_of_str = contents.split("\n");
    println!("Part A:{}",sum);
    
    //Part B
    let list_of_str = contents.split("\n"); // Reseting the interator
    let mut sum:usize = 0;  
    let mut bags: Vec<&str> = Vec::new();
    for str_num in list_of_str{
        if bags.len() != 3{
            if str_num == ""{
                continue;
            }
            bags.push(str_num);
        }else{
            sum += find_priority(&bags,&priority_map);
            bags.clear();
            bags.push(str_num);
        }
        println!("input:{}, bags: {}",str_num,bags.len());
    }
    //let list_of_str = contents.split("\n");
    println!("Part A:{}",sum);
}

