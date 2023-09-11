use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

use std::collections::HashMap;






fn main() {

    let mut closure_map: HashMap<String, Box<dyn Fn(Vec<String>) -> i32>> = HashMap::new();


    let add_closure: Box<dyn Fn(Vec<String>) -> i32> = Box::new(|args| {
        let nums: Vec<i32> = args.iter().map(|s| s.parse::<i32>().unwrap_or(0)).collect();
        nums.iter().sum()
    });

    closure_map.insert("add".to_string(), add_closure);


    let args2 = vec!["10".to_string(), "5".to_string()];
    let result = closure_map["add"](args2);

    println!("Add result: {}", result);



    let args: Vec<String> = env::args().collect();

    let file_name: &String = &args[1];

    if let Ok(file) = File::open(file_name) {

        let reader = BufReader::new(file);

        // Iterate over each line in the file
        for line in reader.lines() {
            if let Ok(line) = line {
                // Split the line into words by whitespace
                let words: Vec<&str> = line.split_whitespace().collect();

                // Process each word
                for word in words {
                    println!("Word: {}", word);
                }
            }
        }

    } else {
        println!("Cannot open file {}", file_name);
    }


}
