use std::collections::HashMap;
use std::fs;

fn sort(ranges: Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    let mut sorted: Vec<Vec<i64>> = vec![];

    let mut hash: HashMap<i64, i64> = HashMap::new();
    for range in &ranges {
        if hash.contains_key(&range[0]) {
            if range[1] > *hash.get(&range[0]).unwrap() {
                hash.insert(range[0], range[1]);
            }
        } else {
            hash.insert(range[0], range[1]);
        }
    }

    for (key, value) in &hash {
        if sorted.len() == 0 {
            sorted.append(&mut vec![vec![*key, *value]]);
        } else {
            let mut done = false;
            for (i, r) in sorted.iter().enumerate() {
                if *key < r[0] {
                    let mut new_sorted: Vec<Vec<i64>> = vec![];
                    new_sorted = sorted[..i].to_vec();
                    new_sorted.append(&mut vec![vec![*key, *value]]);
                    new_sorted.append(&mut sorted[i..].to_vec());

                    done = true;
                    sorted = new_sorted;
                    break;
                }
            }
            if !done {
                sorted.append(&mut vec![vec![*key, *value]]);
            }
        }
    }

    sorted
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Couldn't read file");

    let mut ranges: Vec<Vec<i64>> = vec![];

    let halves = input.split("\n\n").collect::<Vec<&str>>();

    halves[0]
        .split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .for_each(|s| {
            let mut c: Vec<i64> = vec![];
            for n in s.split("-") {
                c.append(&mut vec![n.parse::<i64>().unwrap()])
            }
            ranges.append(&mut vec![c]);
        });

    let mut ids: Vec<i64> = vec![];

    halves[1]
        .split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .for_each(|s| ids.append(&mut vec![s.parse::<i64>().unwrap()]));

    let mut ans_one = 0;

    let mut ans_two = 0;

    for id in ids {
        for range in &ranges {
            let r = range[0]..=range[1];
            if r.contains(&id) {
                ans_one += 1;
                break;
            }
        }
    }

    let sorted = sort(ranges);

    let mut curr = sorted[0].clone();

    let mut new_ranges: Vec<Vec<i64>> = vec![];

    for i in 1..sorted.len() {
        let comp = sorted[i].clone();

        if comp[0] <= curr[1] + 1 {
            if comp[1] > curr[1] {
                curr = vec![curr[0], comp[1]]
            }
            if i == sorted.len() - 1 {
                new_ranges.append(&mut vec![curr.clone()]);
            }
        } else {
            new_ranges.append(&mut vec![curr.clone()]);
            // println!("appending {:?}", curr);
            curr = comp;
            if i == sorted.len() - 1 {
                new_ranges.append(&mut vec![curr.clone()]);
            }
        }
    }

    for range in &new_ranges {
        ans_two += range[1] - range[0] + 1;
    }

    println!("{}", ans_one);

    println!("{}", ans_two);
}
