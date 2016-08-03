use std::io;
use std::cmp;

fn hour(lines: &Vec<Vec<isize>>, x: usize, y: usize) -> isize {
    lines[y][x] + lines[y][x + 1] + lines[y][x + 2] + lines[y + 1][x + 1] + lines[y + 2][x] +
    lines[y + 2][x + 1] + lines[y + 2][x + 2]
}

fn main() {
    let mut lines: Vec<Vec<isize>> = vec![];
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if input == "" {
            break;
        }
        lines.push(input.split_whitespace().map(|x| x.parse::<isize>().unwrap()).collect());
    }

    let mut max: isize = -70;
    for x in 0..4 {
        for y in 0..4 {
            max = cmp::max(max, hour(&lines, x, y));
        }
    }
    print!("{}", max);
}
