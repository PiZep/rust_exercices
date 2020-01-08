use rand::Rng;
use std::io;
use std::collections::HashMap;

pub fn main() {
    println!("integer_list\n============");
    println!("Enter the number of integers of the list:");

    let mut input = String::new();
    let mut num: u8 = 0;
    let mut range: i64 = 0;
    loop {
        io::stdin().read_line(&mut input)
            .expect("Failed to read the line");

        match input.trim().parse::<u8>() {
            Ok(val) => {
                if val > 0 {
                    num = val;
                    break;
                } else {
                    println!("Answer must be positive.");
                    continue;
                }
            },
            Err(_) => {
                println!("Answer must be an integer.");
                continue;
            }
        };
    }
    input = String::new();
    println!("Enter the range value:");
    loop {
        io::stdin().read_line(&mut input)
            .expect("Failed to read the line");

        match input.trim().parse::<i64>() {
            Ok(val) => {
                if val > 0 {
                    range = val;
                    break;
                } else {
                    println!("Answer must be positive.");
                    continue;
                }
            },
            Err(_) => {
                println!("Answer must be an integer.");
                continue;
            }
        };
    }
    format_results(num, range);
}

fn populate_list(lenght: u8, range: i64) -> Vec<i64> {
    let mut list = Vec::new();
    let mut count = 0;

    while count < lenght {
        list.push(rand::thread_rng().gen_range(-range, range));
        count += 1;
    }
    list.sort_unstable();
    list
}

fn get_mean(list: &Vec<i64>) -> f64 {
    let mut mean: f64 = 0.0;
    for element in list {
        mean += *element as f64;
    }
    mean / (list.len() as f64)
}

fn get_median(list: &Vec<i64>) -> f32 {

    let median_index = list.len() / 2;
    if list.len() % 2 == 0 {
        (list[median_index] + list[median_index + 1]) as f32 / 2.0
    } else {
        list[list.len() / 2] as f32
    }
}

fn get_mode(list: &Vec<i64>) -> i64 {
    let mut values: HashMap<i64, i64> = HashMap::new();
    let mut max_count: i64 = 0;
    let mut max: i64 = 0;

    for element in list {
        values.entry(*element).or_insert(0);
        values.insert(*element, values.get(element).unwrap() + 1);
        let count = values.get(element).unwrap();
        if count > &max_count {
            max_count = *count;
            max = *element;
        }
    }
    max
}

fn format_results(num: u8, range: i64) {

    let list = populate_list(num, range);
    println!("\n\nThe list is: {:?}", list);
    println!("The mean of the list is: {}", get_mean(&list));
    println!("The median of the list is: {}", get_median(&list));
    println!("The mode of the list is: {}\n\n", get_mode(&list));
}
