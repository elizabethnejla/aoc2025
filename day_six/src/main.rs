use std::fs;
fn make_a_grid(input: String) -> Vec<Vec<String>> {
    let mut grid: Vec<Vec<String>> = vec![];
    for line in input.split("\n") {
        let mut row: Vec<String> = vec![];
        for elem in line.split_ascii_whitespace() {
            row.append(&mut vec![String::from(elem)]);
        }

        grid.append(&mut vec![row]);
    }

    grid
}

fn make_cephalopod_problems(input: String) -> Vec<Vec<String>> {
    let mut problems: Vec<Vec<String>> = vec![];
    let mut grid: Vec<Vec<char>> = vec![];

    for line in input.split("\n") {
        let mut row: Vec<char> = vec![];

        for c in line.chars() {
            row.append(&mut vec![c])
        }

        grid.append(&mut vec![row])
    }

    let mut curr_problem: Vec<String> = vec![];

    for x in 1..=grid[0].len() {
        let mut num_string: Vec<char> = vec![];
        let mut empty = true;

        for y in 0..grid.len() {
            if grid[y][grid[0].len() - x] != ' ' {
                if y == grid.len() - 1 {
                    curr_problem.append(&mut vec![num_string.iter().collect::<String>()]);
                    num_string = vec![]
                }
                num_string.append(&mut vec![grid[y][grid[0].len() - x]]);
                empty = false;
            }
        }

        if empty {
            problems.append(&mut vec![curr_problem]);
            curr_problem = vec![]
        } else {
            curr_problem.append(&mut vec![num_string.iter().collect::<String>()]);
        }
    }

    problems.append(&mut vec![curr_problem]);

    problems
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Couldn't read file");

    let grid = make_a_grid(input.clone());
    // println!("{:?}", grid);

    let problems = make_cephalopod_problems(input);
    // println!("{:?}", problems);

    let mut ans_one: i64 = 0;

    let mut ans_two: i64 = 0;

    for x in 0..grid[0].len() {
        let op = &grid[grid.len() - 1][x];
        // println!("{}", op);

        match op.as_str() {
            "+" => {
                let mut total = grid[0][x].parse::<i64>().unwrap();
                for n in 1..grid.len() - 1 {
                    // println!("{}", grid[n][x]);
                    total += grid[n][x].parse::<i64>().unwrap();
                }
                ans_one += total;
                // println!("{}", total)
            }
            "*" => {
                let mut total = grid[0][x].parse::<i64>().unwrap();
                for n in 1..grid.len() - 1 {
                    // println!("{}", grid[n][x]);
                    total *= grid[n][x].parse::<i64>().unwrap();
                }
                ans_one += total;
                // println!("{}", total)
            }
            _ => (),
        }
    }

    for problem in problems {
        let op = &problem[problem.len() - 1];
        match op.as_str() {
            "+" => {
                let mut total = problem[0].parse::<i64>().unwrap();
                for i in 1..problem.len() - 1 {
                    total += problem[i].parse::<i64>().unwrap()
                }
                ans_two += total;
                // println!("{}", total)
            }
            "*" => {
                let mut total = problem[0].parse::<i64>().unwrap();
                for i in 1..problem.len() - 1 {
                    total *= problem[i].parse::<i64>().unwrap()
                }
                ans_two += total;
                // println!("{}", total)
            }
            _ => (),
        }
    }

    println!("{}", ans_one);
    println!("{}", ans_two)
}
