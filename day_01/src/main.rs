use std::fs;
use std::str::FromStr;

fn part_b(list_of_str:std::str::Split<&str>) -> [usize;3]{
    //Part B
    let mut current:usize = 0;
    let mut max_array: [usize;3] = [0;3];
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
            current += usize::from_str(str_num).unwrap_or(0);
        }
    }
    //let total:usize = max_array.iter().sum();
    return max_array
}

fn part_a(list_of_str:std::str::Split<&str>) -> usize{
    //Part A
    return part_b(list_of_str)[0];
}

fn main() {
    let file_path:&str = "input.txt";
    let contents:String = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let list_of_str:std::str::Split<&str> = contents.split("\n");

    println!("Part A:{}",part_a(list_of_str.clone()));
    println!("Part B:{}",part_b(list_of_str.clone()).iter().sum::<usize>());
}
