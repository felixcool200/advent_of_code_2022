use std::fs;
use std::str::FromStr;


fn main() {
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let list_of_str = contents.split("\n");

    //Part A
    let mut max_find:u64 = 0;
    let mut current:u64 = 0;
    
    for str_num in list_of_str{
        if str_num == ""{
            if max_find < current{
                max_find = current;
            }
            current = 0;
        }else{
            current += u64::from_str(str_num).unwrap_or(0);
        }
    }
    let list_of_str = contents.split("\n");
    println!("Max carry:{}",max_find);
    

    //Part B
    current = 0;
    let mut max_array: [u64;3] = [0;3];
    for str_num in list_of_str{
        if str_num == ""{
            for i in 0..3{
                if max_array[i] < current{
                    let tmp = max_array[i];
                    max_array[i] = current;
                    current = tmp;
                }
            }
            current = 0;
        }else{
            current += u64::from_str(str_num).unwrap_or(0);
        }
    }

    let total:u64 = max_array.iter().sum();
    println!("Total:{}",total);
}
