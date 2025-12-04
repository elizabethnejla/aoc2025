use std::fs;

fn make_a_grid(input: String) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = vec![vec![]];

    let filler_line = vec!['.'].repeat(input.split("\n").collect::<Vec<&str>>()[0].len() + 2);

    grid.append(&mut vec![filler_line.clone()]);

    for line in input.split("\n") {
        let mut row: Vec<char> = vec![];
        row.append(&mut vec!['.']);

        for c in line.chars() {
            row.append(&mut vec![c])
        }

        row.append(&mut vec!['.']);
        grid.append(&mut vec![row])
    }

    grid.append(&mut vec![filler_line]);

    grid
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Couldn't read file");

    let mut grid = make_a_grid(input);

    let mut ans_one = 0;

    let mut ans_two = 0;

    for (y, r) in grid.iter().enumerate() {
        for (x, _) in r.iter().enumerate() {
            if grid[y][x] == '.' {
                continue;
            } else {
                let ul = grid[y - 1][x - 1];
                let uu = grid[y - 1][x];
                let ur = grid[y - 1][x + 1];
                let ll = grid[y][x - 1];
                let rr = grid[y][x + 1];
                let dl = grid[y + 1][x - 1];
                let dd = grid[y + 1][x];
                let dr = grid[y + 1][x + 1];

                let dirs = vec![ul, uu, ur, ll, rr, dl, dd, dr];

                let mut count = 0;

                for d in dirs {
                    if d == '@' {
                        count += 1
                    }
                }

                if count < 4 {
                    ans_one += 1
                }
            }
        }
    }

    loop {
        let mut total = 0;
        let mut new_grid = grid.clone();
        for (y, r) in grid.iter().enumerate() {
            for (x, _) in r.iter().enumerate() {
                if grid[y][x] == '.' {
                    continue;
                } else {
                    let ul = grid[y - 1][x - 1];
                    let uu = grid[y - 1][x];
                    let ur = grid[y - 1][x + 1];
                    let ll = grid[y][x - 1];
                    let rr = grid[y][x + 1];
                    let dl = grid[y + 1][x - 1];
                    let dd = grid[y + 1][x];
                    let dr = grid[y + 1][x + 1];

                    let dirs = vec![ul, uu, ur, ll, rr, dl, dd, dr];

                    let mut count = 0;

                    for d in dirs {
                        if d == '@' {
                            count += 1
                        }
                    }

                    if count < 4 {
                        ans_two += 1;
                        total += 1;
                        new_grid[y][x] = '.'
                    }
                }
            }
        }
        if total == 0 {
            break;
        }
        grid = new_grid;
    }

    println!("{}", ans_one);
    println!("{}", ans_two)
}
