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


    //let args2 = vec!["10".to_string(), "5".to_string()];
   // let result = closure_map["add"](args2);

  //  println!("Add result: {}", result);



    let args: Vec<String> = env::args().collect();

    let file_name: &String = &args[1];

    if let Ok(file) = File::open(file_name) {

        let reader = BufReader::new(file);

        // Iterate over each line in the file
        for (index, line) in reader.lines().enumerate() {
            if let Ok(line) = line {
                // Split the line into words by whitespace
                let mut words: Vec<String> = line
                    .split_whitespace()
                    .map(|word| word.to_string())
                    .collect();

                if let Some(first_word) = words.get(0).cloned() {
                    let result: i32 = closure_map[&first_word](words.clone());
                    println!("Line {} Result: {}", index, result);
                    words.remove(0);
                } else {
                    println!("Line {} is broken!", index);
                }
            }
        }

    } else {
        println!("Cannot open file {}", file_name);
    }


}
