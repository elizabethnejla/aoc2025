use std::fs;
fn main() {
    let start = std::time::Instant::now();

    let input = fs::read_to_string("input.txt").expect("Couldn't read file");

    let mut ans = 0;
    let mut ans_two = 0;

    for line in input.split("\n") {
        let mut largest = 0;

        for (i, c) in line.chars().enumerate() {
            for cc in line[(i + 1)..].chars() {
                let s = vec![c, cc].iter().collect::<String>();
                let n: i32 = s.parse().unwrap();
                if n > largest {
                    largest = n
                }
            }
        }

        ans += largest
    }

    for line in input.split("\n") {
        let mut digits: Vec<char> = vec![];
        let chars = line.chars().collect::<Vec<char>>();
        let mut i = 0;

        while i < chars.len() {
            let c = chars[i];

            let wiggle_room = (chars.len() - i) - (12 - digits.len());

            if wiggle_room == 0 {
                digits.append(&mut chars[i..].to_vec());
                break;
            }

            let mut l = c.to_digit(10).unwrap(); //largest as number
            let mut lc: char = c; //largest as char
            let mut li = i; //index of largest
            for x in 1..=wiggle_room {
                if chars[i + x].to_digit(10).unwrap() > l {
                    l = chars[i + x].to_digit(10).unwrap();
                    li = i + x;
                    lc = chars[i + x];
                }
            }

            digits.append(&mut vec![lc]);
            if digits.len() == 12 {
                break;
            }

            i = li + 1
        }

        let ns = digits.iter().collect::<String>();
        let a: i64 = ns.parse().unwrap();
        ans_two += a
    }

    println!("{}", ans);
    println!("{}", ans_two);
    println!("{:?}", start.elapsed());
}
