use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

fn main() {
    //part1();
    part2();
}

fn c_to_p(c: char) -> u32{
    let mut a = c as u32;
    if a > 96{
        a = a - 96;
    } else {
        a = a - 38;
    }
    return a;
}

fn part1(){
    let f = File::open("input.txt").expect("File cannot be opened");
    let f_reader = BufReader::new(f);

    let mut score = 0;
    for l in f_reader.lines(){
        let line = l.unwrap();
        //in retrospect I probably should've just iterated through the string
        let set1 : HashSet<char> = HashSet::from_iter(&mut(line[0..(line.len()/2usize)]).chars());
        let set2 : HashSet<char> = HashSet::from_iter(&mut(line[(line.len()/2usize)..(line.len())]).chars());
        let i = set1.intersection(&set2);
        for c in i.into_iter(){
            score += c_to_p(*c); 
        }
    }
    println!("part1: {}", score);
}

fn part2(){
    let f = File::open("input.txt").expect("File cannot be opened");
    let f_reader = BufReader::new(f);

    let mut score = 0;
    let mut counter = 0;
    let mut vec = vec!();
    for l in f_reader.lines(){
        let line = l.unwrap();
        counter += 1;
        vec.push(line);

        if counter == 3 {
            let set1 : HashSet<char> = HashSet::from_iter(vec[0].chars());           
            let set2 : HashSet<char> = HashSet::from_iter(vec[1].chars());
            let set3 : HashSet<char> = HashSet::from_iter(vec[2].chars());

            let i = set2.intersection(&set3);
            for c in i.into_iter(){
                if set1.contains(c) {
                    score += c_to_p(*c);
                }
            }
            counter = 0;
            vec = vec!();
        }
    }
    println!("part2: {score}");
}
