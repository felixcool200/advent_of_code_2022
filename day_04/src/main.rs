use std::fs;
use std::str::FromStr;

//Part A
fn part_a(list_of_str:std::str::Split<&str>) -> usize{
    let mut sum:usize = 0;  
    for str_num in list_of_str{
        if str_num == ""{
            continue;
        }
        let work_times:[[usize;2];2] = create_work_times(str_num);
        
        if work_times[0][0] <= work_times[1][0] && work_times[0][1] >= work_times[1][1]{
            sum += 1;
        }
        else if work_times[0][0] >= work_times[1][0] && work_times[0][1] <= work_times[1][1]{
            sum += 1;
        }
    } 
    return sum;
}

fn create_work_times(str_num:&str) -> [[usize;2];2]{
    let cleaners:Vec<&str> = str_num.split(",").collect();
    let mut work_times: [[usize;2];2] = [[0;2];2];
    let work_times_1_str:Vec<&str> = cleaners[0].split("-").collect();
    let work_times_2_str:Vec<&str> = cleaners[1].split("-").collect();
    
    work_times[0][0] = usize::from_str(work_times_1_str[0]).expect("Not a correct input");
    work_times[0][1] = usize::from_str(work_times_1_str[1]).expect("Not a correct input");
    
    work_times[1][0] = usize::from_str(work_times_2_str[0]).expect("Not a correct input");
    work_times[1][1] = usize::from_str(work_times_2_str[1]).expect("Not a correct input");
    return work_times;
    
}

//Part B
fn part_b(list_of_str:std::str::Split<&str>) -> usize{
    let mut sum:usize = 0;  
    for str_num in list_of_str{
        if str_num == ""{
            continue;
        }

        let work_times:[[usize;2];2] = create_work_times(str_num);
        if work_times[0][1] >= work_times[1][0] && work_times[0][0] <= work_times[1][0]{
            sum += 1;
        }
        else if work_times[1][1] >= work_times[0][0] && work_times[1][0] <= work_times[0][0]{
            sum += 1;
        }
    } 
    return sum;
}

fn read_input(file_path: &str) -> String{
    let contents:String = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    return contents
}

fn main() {
    let file_path:&str = "input.txt";
    let contents:String = read_input(file_path);
    let list_of_str:std::str::Split<&str> = contents.split("\n");

    println!("Part A:{}",part_a(list_of_str.clone()));
    println!("Part B:{}",part_b(list_of_str.clone()));

}

