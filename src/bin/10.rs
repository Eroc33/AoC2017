fn flip_slice<T>(slice: &mut[T], start: usize, len: usize){
    if len == 0 {
        return;
    }
    let mut first = start % slice.len();
    let mut last = (first + (len-1)) % slice.len();
    for _ in 0..(len/2){
        slice.swap(first,last);

        first = (first + 1) % slice.len();
        if last == 0 {
            last = slice.len() - 1;
        }else{
            last = (last - 1) % slice.len();
        }
    }
}

fn part1(){
    let lengths: Vec<usize> = vec![183,0,31,146,254,240,223,150,2,206,161,1,255,232,199,88];
    let mut skip_size = 0;
    let mut pos = 0;
    let mut input: Vec<_> = (0..256).into_iter().collect();
    for len in lengths{
        flip_slice(input.as_mut_slice(),pos,len);
        pos += len + skip_size;
        skip_size += 1;
    }
    println!("{:?}",input);
    println!("{}",input[0]*input[1]);
}

fn part2(){
    let lengths: Vec<usize> = "183,0,31,146,254,240,223,150,2,206,161,1,255,232,199,88".bytes().map(|byte| byte as usize).chain(vec![17, 31, 73, 47, 23].into_iter()).collect();
    let mut skip_size = 0;
    let mut pos = 0;
    let mut input: Vec<_> = (0..256).into_iter().collect();
    for _ in 0..64{
        for &len in &lengths{
            flip_slice(input.as_mut_slice(),pos,len);
            pos += len + skip_size;
            skip_size += 1;
        }
    }

    let mut dense = [0;16];
    for i in 0..16{
        let j = i*16;
        dense[i] = input[j..j+16].iter().fold(0,|a,b| a^b);
    }
    println!("{:?}",input);
    for byte in &dense{
        print!("{:02x}",byte);
    }
    println!();
    println!("{}",input[0]*input[1]);
}

fn main(){
    part2();   
}