use std::io;

fn main() {
    let mut input_n = String::new();
    let mut input_nums = String::new();
    io::stdin().read_line(&mut input_n).unwrap();
    io::stdin().read_line(&mut input_nums).unwrap();
    print!("{}",
           input_nums.split_whitespace().rev().collect::<Vec<&str>>().join(" "));
}
