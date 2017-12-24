use std::collections::HashMap;
use std::io::Read;

enum Op{
    Inc,
    Dec
}

enum CondOp{
    Gt,
    Lt,
    Ge,
    Le,
    Ne,
    Eq
}

struct Cond<'a>{
    reg: &'a str,
    op: CondOp,
    amt: i64
}

impl<'a> Cond<'a>{
    fn check(&self, regs: &mut HashMap<&'a str,i64>) -> bool{
        let tgt = *regs.entry(self.reg).or_insert(0);
        match self.op {
            CondOp::Gt => tgt > self.amt,
            CondOp::Lt => tgt < self.amt,
            CondOp::Ge => tgt >= self.amt,
            CondOp::Le => tgt <= self.amt,
            CondOp::Ne => tgt != self.amt,
            CondOp::Eq => tgt == self.amt,
        }
    }
}

struct Inst<'a>{
    reg: &'a str,
    op: Op,
    amt: i64,
    cond: Cond<'a>
}

impl<'a> Inst<'a>{
    fn execute(&self, regs: &mut HashMap<&'a str,i64>){
        if self.cond.check(regs) {
            let tgt = regs.entry(self.reg).or_insert(0);
            match self.op{
                Op::Inc => *tgt += self.amt,
                Op::Dec => *tgt -= self.amt
            }
        }
    }
}

fn parse_instructions<'a>(input: &'a str) -> Vec<Inst<'a>>
{
    input.lines().map(|line|{
        let mut parts = line.split(' ');
        let reg = parts.next().unwrap();
        let op = match parts.next().unwrap(){
            "inc" => Op::Inc,
            "dec" => Op::Dec,
            &_ => panic!("Unexpected asm op")
        };
        let amt = parts.next().unwrap().parse().unwrap();
        assert_eq!("if",parts.next().unwrap());
        let op_reg = parts.next().unwrap();
        let op_op = match parts.next().unwrap(){
            ">" => CondOp::Gt,
            "<" => CondOp::Lt,
            ">=" => CondOp::Ge,
            "<=" => CondOp::Le,
            "!=" => CondOp::Ne,
            "==" => CondOp::Eq,
            &_ => panic!("Unexpected condition op")
        };
        let op_amt = parts.next().unwrap().parse().unwrap();

        Inst{
            reg,
            op,
            amt,
            cond: Cond{
                reg: op_reg,
                op: op_op,
                amt: op_amt,
            }
        }
    }).collect()
}

fn main(){
    let mut input_file = std::fs::File::open("8.input").expect("Failed to open input file");
    let mut input = String::new();
    input_file.read_to_string(&mut input).expect("Failed to read input file");
    let insts = parse_instructions(input.as_str());
    let mut regs = HashMap::new();
    let mut max_reg: Option<(&str,i64)> = None;
    for inst in insts{
        inst.execute(&mut regs);
        let cur_max = regs.iter().max_by_key(|&(k,v)| v).map(|(k,v)| (k.clone(),v.clone())).expect("Must be a max value");
        if let Some(prev_max) = max_reg{
            if cur_max.1 > prev_max.1{
                max_reg = Some(cur_max);
            }
        }else{
            max_reg = Some(cur_max);
        }
    }
    //part 1
    let max_end_reg = regs.into_iter().max_by_key(|&(k,v)| v).expect("Must be a max value");
    println!("max reg: {:?}",max_end_reg);
    //part 2
    println!("max reg all_time: {:?}",max_reg);
}