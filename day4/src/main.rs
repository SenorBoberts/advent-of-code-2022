use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

fn main() {
    println!("part1: {}", file_processor(&check_enclosing));
    println!("part2: {}", file_processor(&check_overlap));
}

fn file_processor(fun: &dyn Fn(&str, &str, &str, &str) -> bool) -> i32{
    let f = File::open("input").expect("File cannot be opened");
    let f_reader = BufReader::new(f);

    let re = Regex::new(r"^(\d{1,})-(\d{1,}),(\d{1,})-(\d{1,})$").unwrap();
    let mut count = 0;
    for l in f_reader.lines(){
        let line = l.unwrap();
        
        let cap = re.captures(&line).unwrap();
        if fun(&cap[1], &cap[2], &cap[3], &cap[4]){
            count += 1;
        } 
    }
    return count;
}

fn check_enclosing(a: &str, b: &str, x: &str, y: &str) -> bool{
    let na : i32 = a.parse().unwrap();
    let nb : i32 = b.parse().unwrap();
    let nx : i32 = x.parse().unwrap();
    let ny : i32 = y.parse().unwrap();
    if nb - na > ny - nx {
        if na <= nx && nb >= ny{
            return true;
        }else{
            return false;
        }
    }else{
        if nx <= na && ny >= nb{
            return true;
        }else{
            return false;
        }
    }
}

fn check_overlap(a: &str, b: &str, x: &str, y: &str) -> bool{
    let na : i32 = a.parse().unwrap();
    let nb : i32 = b.parse().unwrap();
    let nx : i32 = x.parse().unwrap();
    let ny : i32 = y.parse().unwrap();
    if na <= ny && nx <= nb{
        return true;
    }else{
        return false;
    }
}
