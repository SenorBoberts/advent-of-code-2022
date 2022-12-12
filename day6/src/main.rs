use::std::fs::File;
use::std::io::prelude::*;


fn check_unique(s: &str) -> bool{
    for i in 0..s.len(){
        for j in i+1..s.len(){
            if &s[i..=i] == &s[j..=j] {
                return false;
            }
        }
    }
    return true;
}

fn process_message(l: usize){
    let mut f = File::open("input").expect("File cannot be opened");
    let mut s = String::new();
    f.read_to_string(&mut s).expect("failed to read"); 
    
    for i in 0..s.len()-l{
        if check_unique(&s[i..i+l]) {
            println!("{}, at {}", &s[i..i+l], i + l);
            break;
        }
    }
}

fn main(){
    process_message(4usize);
    process_message(14usize);
}
