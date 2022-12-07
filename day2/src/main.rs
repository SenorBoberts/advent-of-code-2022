use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part1();
    part2();
}


fn part2(){
    let f = File::open("input.txt").expect("File open failed");
    let f_reader = BufReader::new(f);

    let mut score = 0;
    for l in f_reader.lines(){
        let line = l.unwrap();

        
        match (&line[0..1], &line[2..3]){
            ("A", "X") => score += 3,
            ("A", "Y") => score += 4,
            ("A", "Z") => score += 8,
            ("B", "X")=> score += 1,
            ("B", "Y")=> score += 5,
            ("B", "Z")=> score += 9,
            ("C", "X" )=> score += 2,
            ("C", "Y" )=> score += 6,
            ("C", "Z" )=> score += 7,
            _ => println!("not valid")
        }

    }
    println!("{}", score);
}

fn part1(){
    let f = File::open("input.txt").expect("File open failed");
    let f_reader = BufReader::new(f);

    let mut score = 0;
    for l in f_reader.lines(){
        let line = l.unwrap();
        
        match (&line[0..1], &line[2..3]){
            ("A", "X") => score += 4,
            ("A", "Y") => score += 8,
            ("A", "Z") => score += 3,
            ("B", "X")=> score += 1,
            ("B", "Y")=> score += 5,
            ("B", "Z")=> score += 9,
            ("C", "X" )=> score += 7,
            ("C", "Y" )=> score += 2,
            ("C", "Z" )=> score += 6,
            _ => println!("not valid")
        }
    
    }

    println!("{}", score);
}
