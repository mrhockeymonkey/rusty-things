use std::env;
use std::fs;
use std::io::{Read, Write};

// fn process(input_fname: &str, output_fname: &str) -> Result<(), String> {
//     let mut input_file =
//         fs::File::open(input_fname).map_err(|err| format!("error opening input {}: {}", input_fname, err))?;
//     let mut contents = Vec::new();
//     input_file
//         .read_to_end(&mut contents)
//         .map_err(|err| format!("read error: {}", err))?;
//
//     let mut output_file = fs::File::create(output_fname)
//         .map_err(|err| format!("error opening output {}: {}", output_fname, err))?;
//     output_file
//         .write_all(&contents)
//         .map_err(|err| format!("write error: {}", err))
// }

fn main() -> i32 {
    // let args: Vec<String> = env::args().collect();
    // let program = args[0].clone();
    // let a = args[1].clone();
    // let b = args[2].clone();
    //
    // println!("a = {}", a);
    // println!("b = {}", b);

    println!("hello from rs-metric");
    3
}