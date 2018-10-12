
fn main(){

use std::fs::File;
use std::fs::OpenOptions;

let input = "error";

let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open("Nickslist");

file.Write(input);

}



