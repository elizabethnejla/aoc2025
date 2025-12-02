use std::fs;
fn main() {
    let input = fs::read_to_string("input.txt").expect("Couldn't read file");
    let mut ans = 0;
    for line in input.split(",") {
        let ends: Vec<&str> = line.split("-").collect();
        let first: i64 = ends[0].parse().unwrap();
        let mut second: i64 = ends[1].parse().unwrap();
        second += 1;

        for n in first..second {
            let st = n.to_string();
            let s = st.as_str();
            if s.len() % 2 != 0 {
                continue;
            }

            let halves = s.split_at(s.len() / 2);
            if halves.0 == halves.1 {
                ans += n
            }
        }
    }

    let mut ans_two = 0;
    for line in input.split(",") {
        let ends: Vec<&str> = line.split("-").collect();
        let first: i64 = ends[0].parse().unwrap();
        let mut second: i64 = ends[1].parse().unwrap();
        second += 1;
        let mut done = false;

        for n in first..second {
            let st = n.to_string();
            let s = st.as_str();
            let c: Vec<char> = s.chars().collect();
            let mut i = 1;
            while i < c.len() {
                if done {
                    done = false;
                    break;
                }
                if c.len() % (i) != 0 {
                    i += 1;
                    continue;
                }
                let mut s = 0;
                let mut e = i;
                let mut curr = &c[s..e];

                let mut invalid = true;

                while e <= c.len() {
                    s = e;
                    e = e + i;
                    if e > c.len() {
                        break;
                    }
                    let next = &c[s..e];
                    if next != curr {
                        invalid = false;
                        break;
                    }

                    curr = next
                }

                if invalid {
                    ans_two += n;
                    done = true
                }

                i += 1;
            }
        }
    }

    println!("{}", ans);
    println!("{}", ans_two)
}
