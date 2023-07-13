use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let mut l: Vec<&str> = input.trim().split(' ').collect();
    l.sort();

    if l[0] == l[1] {
        println!("D");
    } else if l[0] == "P" {
        if l[1] == "R" {
            println!("P");
        } else {
            println!("S");
        }
    } else {
        println!("R");
    }
}
