use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let l: Vec<&str> = input.trim().split(' ').collect();
    let mut l = l;
    l.sort();

    let mut cnt = 0;
    let mut chk = vec![false; 3];

    for &i in &l {
        match i {
            "R" => {
                if !chk[0] {
                    chk[0] = true;
                    cnt += 1;
                }
            }
            "P" => {
                if !chk[1] {
                    chk[1] = true;
                    cnt += 1;
                }
            }
            "S" => {
                if !chk[2] {
                    chk[2] = true;
                    cnt += 1;
                }
            }
            _ => {}
        }
    }

    if cnt == 1 || cnt == 3 {
        println!("D");
    } else {
        if chk[0] && chk[1] {
            println!("P");
        } else if chk[0] && chk[2] {
            println!("R");
        } else {
            println!("S");
        }
    }
}

