use std::fs::File;
use std::io::{BufRead, BufReader};


fn main(){
    part1();
    part2();
}

fn part2(){
    let f = File::open("input").expect("File open failed");
    let f_reader = BufReader::new(f);

    let mut max = (0i32, 0i32, 0i32);
    let mut cur : i32 = 0;
    for l in f_reader.lines(){
        let line = l.unwrap();
        if line.is_empty() {
            if cur > max.0{
                max.2 = max.1;
                max.1 = max.0;
                max.0 = cur;
            }else if cur > max.1 {
                max.2 = max.1;
                max.1 = cur;
            }else if cur > max.2{
                max.2 = cur;
            }
            cur = 0;
            //println!("{},{},{}", max.0, max.1, max.2);
        }else{
            cur += line.parse::<i32>().unwrap();
        }
    }

    println!("part 2: {}", max.0 + max.1 + max.2);

}

fn part1() {
    let f = File::open("input").expect("File open failed");
    let f_reader = BufReader::new(f);

    let mut max : i32 = 0;
    let mut cur : i32 = 0;
    for l in f_reader.lines(){
        let line = l.unwrap();
        
        if line.is_empty() {
            if cur > max {
                max = cur;
            }
            cur = 0;
        }else{
            cur += line.parse::<i32>().unwrap();
        }
    }

    println!("part 1: {}", max);
}
