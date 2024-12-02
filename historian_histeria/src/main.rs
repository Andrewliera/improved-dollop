use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
//use std::collections::HashMap;

fn main() {
    println!("Advent Of code 2024 solver for day 1!");

    let path = "list.txt";

    let mut list_1: Vec<u64> = Vec::new();
    let mut list_2: Vec<u64> = Vec::new();
    let mut list_3: Vec<u64> = Vec::new();
    let mut list_4: Vec<u64> = Vec::new();

    if let Ok(lines) = read_lines(path){
    for line in lines.flatten() {

        let parts: Vec<u64> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        
            list_1.push(parts[0]);
            list_3.push(parts[0]);
            list_2.push(parts[1]);
            list_4.push(parts[1]);
        }
    }
    


    let result = find_distance_between_elements(&mut list_1, &mut list_2);
    let result2 = find_similarity_between_elements(&mut list_3, &mut list_4);

    print!("Result from list: {}\n\n", result);
    print!("Result from list: {}\n\n", result2);

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn find_distance_between_elements(list_1: &mut Vec<u64>, list_2: &mut Vec<u64>) -> u64{
    list_1.sort();
    list_2.sort();

    let mut i = list_1.len();
    let mut a;
    let mut b;
    let mut result = 0;
    while i != 0 {

        a = list_1.pop().unwrap();
        b = list_2.pop().unwrap();

        let diff =  if a > b { a - b } else { b - a };
        result += diff;
        i -= 1;
    }
    return result;
}

fn find_similarity_between_elements(list_1: &mut Vec<u64>, list_2: &mut Vec<u64>) -> u64 {
    list_1
        .iter()
        .map(|left_elt| {
            let count = list_2
            .iter()
            .filter(|right_elt| *right_elt == left_elt)
            .count();
            left_elt * u64::try_from(count).unwrap()
        })
        .sum()
}
