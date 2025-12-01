use std::fs;
use std::fs::read_to_string;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Couldn't read file");
    let mut ans = 0;
    let mut ans_two = 0;
    let mut pos: i64 = 50;
    for line in input.split("\n") {
        if pos == 0 {
            ans += 1;
        }
        match line.chars().nth(0).unwrap() {
            'L' => {
                if pos == 0 {
                    ans_two -= 1
                }
                let num: i64 = line.strip_prefix("L").unwrap().parse().unwrap();
                pos -= num;
                if pos == 0 {
                    ans_two += 1;
                }
                while pos < 0 {
                    ans_two += 1;
                    pos = 100 + pos;
                    if pos == 0 {
                        ans_two += 1
                    }
                }
            }
            'R' => {
                let num: i64 = line.strip_prefix("R").unwrap().parse().unwrap();
                pos += num;
                if pos == 0 {
                    ans_two += 1;
                }
                while pos > 99 {
                    ans_two += 1;
                    pos = pos - 100;
                }
            }
            _ => (),
        }
    }

    println!("{}", ans);
    println!("{}", ans_two)
}
