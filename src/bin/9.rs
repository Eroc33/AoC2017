#![feature(io)]
use std::io::Read;

fn main(){
    let mut input_file = std::fs::File::open("9.input").expect("Failed to open input file");
    let mut skip_next = false;
    let mut garbage = false;
    let mut nesting = 0;
    let mut score = 0;
    let mut count = 0;
    for ch in input_file.chars(){
        let ch = ch.unwrap();
        if skip_next{
            skip_next = false;
            continue;
        }

        match ch {
            '!' => skip_next = true,
            '<' if !garbage => garbage = true,
            '>' => garbage = false,
            '{' if !garbage => nesting += 1,
            '}' if !garbage => {
                score += nesting;
                nesting -= 1;
            }
            _ => {
                if garbage {
                    count += 1;
                }
            }
        }
    }
    println!("score: {}, count: {}", score, count);
}