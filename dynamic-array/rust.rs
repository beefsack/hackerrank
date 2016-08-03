use std::io;

fn main() {
    let mut first_line = String::new();
    io::stdin().read_line(&mut first_line).unwrap();
    let first_line_nums: Vec<usize> =
        first_line.split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect();
    let n = first_line_nums[0];

    let mut seq_list: Vec<Vec<usize>> = vec![vec![]; n];
    let mut last_ans: usize = 0;

    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        if line == "" {
            break;
        }
        let line_nums: Vec<usize> =
            line.split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect();
        match line_nums[0] {
            1 => seq_list[(line_nums[1] ^ last_ans) % n].push(line_nums[2]),
            2 => {
                let seq_index = (line_nums[1] ^ last_ans) % n;
                last_ans = seq_list[seq_index][line_nums[2] % seq_list[seq_index].len()];
                println!("{}", last_ans);
            }
            _ => panic!("nope"),
        }
    }
}
