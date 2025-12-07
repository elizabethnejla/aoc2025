use std::collections::{HashMap, HashSet};
use std::fs;
use std::hash::Hash;

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Coord {
    y: usize,
    x: usize,
}

fn make_a_grid(input: String) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = vec![];

    for line in input.split("\n") {
        let mut row: Vec<char> = vec![];

        for c in line.chars() {
            row.append(&mut vec![c])
        }

        grid.append(&mut vec![row])
    }

    grid
}

fn extend(grid: &Vec<Vec<char>>, y: usize, x: usize, hash: &mut HashSet<String>) -> i64 {
    for i in 1..grid.len() - y {
        let loc = grid[y + i][x];
        if loc == '^' {
            if hash.contains(&format!("{}-{}", y + i, x)) {
                return 0;
            } else {
                hash.insert(format!("{}-{}", y + i, x));

                return extend(grid, y + i, x - 1, hash) + extend(grid, y + i, x + 1, hash) + 1;
            }
        }
    }

    0
}

fn extend_two(grid: &Vec<Vec<char>>, y: usize, x: usize, hash: &mut HashMap<Coord, i64>) -> i64 {
    for i in 1..grid.len() - y {
        let loc = grid[y + i][x];
        if hash.contains_key(&Coord { y: y + i, x: x }) {
            return *hash.get(&Coord { y: y + i, x: x }).unwrap();
        }
        if loc == '^' {
            let paths = extend_two(grid, y + i, x - 1, hash) + extend_two(grid, y + i, x + 1, hash);
            hash.insert(Coord { y: y + i, x: x }, paths);
            return paths;
        }
    }

    1
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Couldn't find file");

    let grid = make_a_grid(input);

    let mut ans_one = 0;

    let mut ans_two = 0;

    let mut hash: HashSet<String> = HashSet::new();

    let mut hash_two: HashMap<Coord, i64> = HashMap::new();

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == 'S' {
                ans_one = extend(&grid, y, x, &mut hash);
                ans_two = extend_two(&grid, y, x, &mut hash_two)
            }
        }
    }

    println!("{}", ans_one);
    println!("{}", ans_two)
}
