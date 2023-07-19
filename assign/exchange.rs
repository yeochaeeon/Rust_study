use std::io;

fn main() {
    let mut euro_str = String::new();
    io::stdin().read_line(&mut euro_str).expect("Failed to read line");
    let euro: i32 = euro_str.trim().parse().expect("Invalid input");

    let mut money_str = String::new();
    io::stdin().read_line(&mut money_str).expect("Failed to read line");
    let money: i32 = money_str.trim().parse().expect("Invalid input");

    let mut exchange = money / euro;
    if exchange < 100 {
        exchange -= 1;
    }

    let mut l = Vec::new();
    for &i in &[100, 50, 10, 1] {
        l.push(exchange / i);
        exchange %= i;
    }

    println!("{} {} {} {}", l[0], l[1], l[2], l[3]);
}


