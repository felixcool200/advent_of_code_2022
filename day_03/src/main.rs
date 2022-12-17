use std::fs;

fn main() {
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
            for letter2 in bag2.chars(){
                if letter == letter2{
                    sum += (letter as usize) - 25;
                }
            }
        }
        //Fix new line in input from VIM
        if str_num == ""{
            continue;
        }
        println!("{}",sum);
    }
    //let list_of_str = contents.split("\n");
    println!("Score:{}",sum);
    
}

