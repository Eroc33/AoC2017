use std::collections::{HashMap,VecDeque};
use std::rc::Rc;
use std::cell::RefCell;
use std::io::Read;

const INPUT: &str = "pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)";

#[derive(Debug)]
struct Edge<'a>{
    from: &'a str,
    to: &'a str
}

#[derive(Debug)]
struct Program<'a>{
    name: &'a str,
    weight: u32,
}

fn parse_input<'a>(input: &'a str) -> (Vec<Program<'a>>,Vec<Edge<'a>>){
    let mut programs = Vec::with_capacity(input.lines().count());
    let mut edges = Vec::with_capacity(programs.capacity());
    for line in input.lines(){
        let mut line = line.split("->");
        let (name,weight) = {
            let name_weight = line.next().unwrap();
            let mut name_weight = name_weight.split(" ");
            let name = name_weight.next().unwrap();
            let weight = u32::from_str_radix(name_weight.next().unwrap().trim_matches(&['(',')'][..]),10).unwrap();
            (name,weight)
        };
        line.next().iter().flat_map(|strn|{
            strn.split(",").map(str::trim)
        }).for_each(|child|{
            edges.push(Edge{from: name, to: child});
        });
        programs.push(Program{
            name,
            weight
        });
    }
    (programs,edges)
}

fn find_root<'a,'b>(programs: &'b Vec<Program<'a>>,edges: &Vec<Edge<'a>>) -> Option<&'b Program<'a>>{
    for program in programs{
        if !edges.iter().any(|edge| edge.to == program.name){
            return Some(program);
        }
    }
    None
}

fn find_leaves<'a,'b>(programs: &'b Vec<Program<'a>>,edges: &Vec<Edge<'a>>) -> Vec<&'b Program<'a>>{
    programs.iter().filter(|program|{
        !edges.iter().any(|edge| edge.from == program.name)
    }).collect()
}


fn part1<'a>(programs: Vec<Program<'a>>,edges: Vec<Edge<'a>>){
    if let Some(program) = find_root(&programs,&edges){
        println!("Root: {}",program.name);
    }else{
        println!("No root found");
    }
}

fn calc_weights<'a>(root: &Program<'a>, programs: &Vec<Program<'a>>,edges: &Vec<Edge<'a>>, weight_sums: &mut HashMap<&'a str,(u32,Vec<(u32,&'a str)>)>) -> u32{
    if let Some(&(ref weight,ref child_weights)) = weight_sums.get(root.name){
        return weight + child_weights.iter().map(|&(w,_n)| w).sum::<u32>();
    }

    let child_weights: Vec<_> = edges.iter()
            .filter(|edge| edge.from == root.name)
            .map(|edge| programs.iter().find(|prog| prog.name == edge.to).unwrap())
            .map(|prog|{
                (calc_weights(prog,programs,edges,weight_sums),prog.name)
            })
            .collect();

    let full_weight = root.weight + child_weights.iter().map(|&(w,_n)| w).sum::<u32>();
    weight_sums.insert(root.name,(root.weight,child_weights));
    return full_weight;
}

fn part2<'a>(programs: Vec<Program<'a>>,edges: Vec<Edge<'a>>){
    let mut weight_sums: HashMap<&str,(u32,Vec<(u32,&str)>)> = HashMap::new();
    for leaf in find_leaves(&programs,&edges){
        weight_sums.insert(leaf.name,(leaf.weight,vec![]));
    }

    let mut queue = VecDeque::from(find_leaves(&programs,&edges));
    let mut quit = false;
    while let Some(node) = queue.pop_front(){
        edges.iter()
            .find(|edge| edge.to == node.name )
            .map(|edge| edge.from )
            .map(|parent_name| programs.iter().find(|prog| prog.name == parent_name).unwrap())
            .map(|parent| queue.push_back(parent) );

        calc_weights(node, &programs, &edges, &mut weight_sums);

        let child_weights = &weight_sums.get(node.name).expect("Should have just calc-ed child weights").1;

        if let Some(different) = child_weights.iter().find(|&&(weight,_)| weight != child_weights[0].0){
            let diff = (different.0 as i32-child_weights[0].0 as i32).abs();
            println!("At {} children of mismatched weights {:?}",node.name,child_weights);
            println!("(should be a difference of {}, {:?} vs {:?})",diff,programs.iter().find(|node| node.name == child_weights[0].1),programs.iter().find(|node| node.name == different.1));
            return;
        }
    }
    println!("No mismatch found")   
}

fn main() {
    let mut input_file = std::fs::File::open("7.input").expect("Failed to open input file");
    let mut input = String::with_capacity(30*1024);//my input is ~29k
    input_file.read_to_string(&mut input).expect("Failed to read input file");
    let (programs,edges) = parse_input(input.as_str());
    //part1(programs,edges);
    part2(programs,edges);
}