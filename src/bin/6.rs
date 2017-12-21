use std::collections::{HashSet,HashMap};

#[derive(PartialEq,Eq)]
enum Control<T>{
    Continue,
    Return(T),
}

fn part1<I: Into<Vec<u32>>>(banks: I) -> usize{
    let banks = banks.into();
    let mut seen_configs = HashSet::new();
    seen_configs.insert(banks.clone());
    run(banks,|bank: &Vec<u32>,its|{
        if seen_configs.contains(bank){
            Control::Return(its)
        }else{
            seen_configs.insert(bank.clone());
            Control::Continue
        }
    })
}

fn part2<I: Into<Vec<u32>>>(banks: I) -> usize{
    let banks = banks.into();
    let mut seen_configs = HashMap::new();
    seen_configs.insert(banks.clone(),0);
    run(banks,|bank: &Vec<u32>,its|{
        if seen_configs.contains_key(bank){
            Control::Return(its - seen_configs.get(bank).unwrap())
        }else{
            seen_configs.insert(bank.clone(),its);
            Control::Continue
        }
    })
}

fn run<I, F>(banks: I, mut control_fn: F) -> usize
    where I: Into<Vec<u32>>,
          F: for<'a> FnMut(&'a Vec<u32>,usize) -> Control<usize>
{
    let mut banks = banks.into();
    let mut iterations = 0;
    loop{
        //reverse the iterator here as we want to take the first bank for any
        //tie, whereas max_by_key takes the last tie
        let max_idx = banks.iter().enumerate().rev().max_by_key(|&(_i,val)| val).map(|(i,_val)|i).unwrap();
        let mut count = std::mem::replace(&mut banks[max_idx],0);
        let mut idx = (max_idx + 1) % banks.len();
        while count > 0 {
            banks[idx] += 1;
            idx = (idx + 1) % banks.len();
            count -= 1;
        }
        iterations += 1;
        if let Control::Return(t) = control_fn(&banks,iterations){
            return t;
        }
    }
}

const INPUT: &str = "0	5	10	0	11	14	13	4	11	8	8	7	1	4	12	11";

fn main() {
    println!("{}",part2(INPUT.split("\t").map(|i| u32::from_str_radix(i,10).unwrap()).collect::<Vec<_>>()));
}