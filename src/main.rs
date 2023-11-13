// Open file
// Read file line by line
// De-space every line
// Get each monogram, bigram, trigram and word from each line
// Put into a hashmap, if it isn't already there insert it with the default value of 1, else increment the value by 1
// Output the hashmaps, properly, into a json file

use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashMap;

fn despace(s: &String) -> String {
    let x = s.replace(" ", "");
    x
}

fn vectorize(s: &String) -> Vec<char> {
    let mut v = vec![];
    
    for i in s.chars() {
        v.push(i);
    }
    v
}


fn main() -> io::Result<()> {
    let file = File::open(r"C:\Users\Yello\Downloads\mr(2).txt")?;
    let reader = BufReader::new(file);
    // let mut bigrams: HashMap<&str, i32> = HashMap::new();

    for line in reader.lines() {
        let x = line.unwrap();
        let v = vectorize(&despace(&x));
        let bigrams = v.windows(2);
        println!("{:?}", bigrams);
    }

    Ok(())
}
