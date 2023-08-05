use std::io;

fn main() {
    let mut month_len = String::new();
    io::stdin().read_line(&mut month_len).expect("Failed to read input");
    let month_len: i32 = month_len.trim().parse().expect("Invalid input");

    let mut day_list = String::new();
    io::stdin().read_line(&mut day_list).expect("Failed to read input");
    let day_list: Vec<i32> = day_list
        .split_whitespace()
        .map(|x| x.parse().expect("Invalid input"))
        .collect();

    let mut firstday = String::new();
    io::stdin().read_line(&mut firstday).expect("Failed to read input");

    let day_char = ["일", "월", "화", "수", "목", "금", "토"];

    for _ in 0..3 {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).expect("Failed to read input");
        let mut iter = input_line.split_whitespace();
        let month: i32 = iter.next().unwrap().parse().expect("Invalid input");
        let day: i32 = iter.next().unwrap().parse().expect("Invalid input");

        if month_len < month || day_list[month as usize - 1] < day {
            println!("땡");
        } else {
            let mut sum_days = 0;
            for (index, k) in day_char.iter().enumerate() {
                if k == &firstday.trim() {
                    sum_days += index as i32 - 1;
                    break;
                }
            }
            let result_day = (sum_days + day_list.iter().take((month - 1) as usize).sum::<i32>() + day) % 7;
            println!("{}", day_char[result_day as usize]);
        }
    }
}
