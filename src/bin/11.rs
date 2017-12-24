use std::io::Read;

fn str_to_dir(i: &str) -> (isize,isize,isize)
{
    match i{
        "n"  => ( 0,  1, -1),
        "s"  => ( 0, -1,  1),

        "ne" => ( 1,  0, -1),
        "sw" => (-1,  0,  1),

        "nw" => (-1,  1,  0),
        "se" => ( 1, -1,  0),
        other => panic!("Bad direction {}",other),
    }
}

fn hex_dist(a: (isize,isize,isize), b: (isize,isize,isize)) -> isize{
    (a.0 - b.0).abs().max( (a.1 - b.1).abs() ).max( (a.2 - b.2).abs() )
}

fn main(){
    let mut input_file = std::fs::File::open("11.input").expect("Failed to open input file");
    let mut input = String::with_capacity(25*1024);//my input is ~22k
    input_file.read_to_string(&mut input).expect("Failed to read input file");
    let mut max_dist = 0;
    let pos = input.split(",").map(str::trim).map(str_to_dir).fold((0,0,0),|a,b|{
        let pos = (a.0+b.0,a.1+b.1,a.2+b.2);
        let dist = hex_dist(pos,(0,0,0));
        max_dist = max_dist.max(dist);
        pos
    });
    println!("{}, max: {}",hex_dist(pos,(0,0,0)),max_dist);
}