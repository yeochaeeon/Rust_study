use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let l: Vec<i32> = input.trim().split(' ').map(|x| x.parse().expect("Invalid input")).collect();
    let mut l = l;
    l.sort();

    if l[2] >= l[1] + l[0] {
        println!("0");
    } else {
        if l[2].pow(2) == l[1].pow(2) + l[0].pow(2) {
            println!("1");
        } else if l[2].pow(2) > l[1].pow(2) + l[0].pow(2) {
            println!("2");
        } else {
            println!("3");
        }
    }
}

