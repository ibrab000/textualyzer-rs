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

fn main() -> io::Result<()> {
    let file = File::open(input_file)?;
    let reader = BufReader::new(file);
    let mut bigrams: HashMap<&str, i32> = HashMap::new();

    for line in reader.lines() {
        
    }

    Ok(())
}
