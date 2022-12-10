use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::VecDeque;
use regex::Regex;

fn main() {
    part1();
    part2();
}

fn setup_sample() -> Vec<VecDeque<char>> {
    let mut crates : Vec<VecDeque<char>> = vec!();
    crates.push(VecDeque::from(['N', 'Z']));
    crates.push(VecDeque::from(['D', 'C', 'M']));
    crates.push(VecDeque::from(['P']));
    return crates;
}

fn setup_input() -> Vec<VecDeque<char>> {
    let mut crates : Vec<VecDeque<char>> = vec!();
    crates.push(VecDeque::from(['C', 'S', 'G', 'B']));
    crates.push(VecDeque::from(['G', 'V', 'N', 'J', 'H', 'W', 'M', 'T']));
    crates.push(VecDeque::from(['S', 'Q', 'M']));
    crates.push(VecDeque::from(['M', 'N', 'W', 'T', 'L', 'S', 'B']));
    crates.push(VecDeque::from(['P', 'W', 'G', 'V', 'T', 'F', 'Z', 'J']));
    crates.push(VecDeque::from(['S', 'H', 'Q', 'G', 'B', 'T', 'C']));
    crates.push(VecDeque::from(['W', 'B', 'P', 'J', 'T']));
    crates.push(VecDeque::from(['M', 'Q', 'T', 'F', 'Z', 'C', 'D', 'G']));
    crates.push(VecDeque::from(['F', 'P', 'B', 'H', 'S', 'N']));
    return crates;
}

fn move_crate(crates: &mut Vec<VecDeque<char>>, from: usize, to: usize){
    let c = crates[from-1].pop_front().unwrap();
    crates[to-1].push_front(c);
}

fn part1(){
    let mut crates = setup_input(); //setup_sample();
    let f = File::open("input").expect("File cannot be opened");
    let f_reader = BufReader::new(f);

    let re = Regex::new(r"^move (\d{1,}) from (\d{1,}) to (\d{1,})$").unwrap();
    for l in f_reader.lines(){
        let line = l.unwrap();
        let cap = re.captures(&line).unwrap();
        
        for _i in 0..(cap[1].parse().unwrap()){
            move_crate(&mut crates, cap[2].parse().unwrap(), cap[3].parse().unwrap());
        }

    }
    for i in 0..crates.len(){
        print!("{}", crates[i].front().unwrap());
    }
    print!("\n");
}

fn move_crates_together(crates: &mut Vec<VecDeque<char>>, from: usize, to: usize, num: usize){
    let mut tmp_vec = vec!();

    for _i in 0..num{
        tmp_vec.push(crates[from-1].pop_front().unwrap());
    }

    for _i in 0..num{
        crates[to-1].push_front(tmp_vec.pop().unwrap());
    }
}

fn part2(){
    let mut crates = setup_input(); //setup_sample();
    let f = File::open("input").expect("File cannot be opened");
    let f_reader = BufReader::new(f);

    let re = Regex::new(r"^move (\d{1,}) from (\d{1,}) to (\d{1,})$").unwrap();
    for l in f_reader.lines(){
        let line = l.unwrap();
        let cap = re.captures(&line).unwrap();
        move_crates_together(&mut crates, cap[2].parse().unwrap(), cap[3].parse().unwrap(), cap[1].parse().unwrap());

    }
    for i in 0..crates.len(){
        print!("{}", crates[i].front().unwrap());
    }
    print!("\n");

}
