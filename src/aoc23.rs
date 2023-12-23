fn parse(str: &str) -> [[char; N]; N] {
    let mut grid = [['.'; N]; N];
    for (y, line) in str.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid[y][x] = c;
        }
    }
    return grid;
}

fn get_valid_dirs1(c: char) -> Vec<(i32, i32)> {
    match c {
        '.' => vec![(1, 0), (-1, 0), (0, 1), (0, -1)],
        '>' => vec![(1, 0)],
        'v' => vec![(0, 1)],
        '<' => vec![(-1, 0)],
        '^' => vec![(0, -1)],
        _ => todo!(),
    }
}

fn get_longest(get_valid_dirs_func: fn(c: char) -> Vec<(i32, i32)>) -> i32 {
    struct State {
        grid: [[char; N]; N],
        longest: usize,
        len: usize,
        visited: [[bool; N]; N],
        n: i64,
    }
    let mut state = State {
        grid: parse(INPUT),
        longest: 0,
        len: 0,
        visited: [[false; N]; N],
        n: 0,
    };

    fn dfs(
        pos: (usize, usize),
        state: &mut State,
        get_valid_dirs_func: fn(c: char) -> Vec<(i32, i32)>,
    ) {
        state.n += 1;
        if state.n % 100000000 == 0 {
            println!("{}", state.n);
        }
        state.len += 1;
        state.visited[pos.1][pos.0] = true;

        if pos == (N - 2, N - 1) {
            if state.len > state.longest {
                state.longest = state.len;
                println!("longest: {}", state.longest);
            }
        } else {
            let valid_dirs = get_valid_dirs_func(state.grid[pos.1][pos.0]);
            for dir in valid_dirs {
                let pos_next_i = (pos.0 as i32 + dir.0, pos.1 as i32 + dir.1);
                if pos_next_i.0 < 0
                    || pos_next_i.0 >= N as i32
                    || pos_next_i.1 < 0
                    || pos_next_i.1 >= N as i32
                {
                    continue;
                }
                let pos_next = (pos_next_i.0 as usize, pos_next_i.1 as usize);
                if state.visited[pos_next.1][pos_next.0]
                    || state.grid[pos_next.1][pos_next.0] == '#'
                {
                    continue;
                }
                dfs(pos_next, state, get_valid_dirs_func);
            }
        }

        state.visited[pos.1][pos.0] = false;
        state.len -= 1;
    }

    dfs((1, 0), &mut state, get_valid_dirs_func);

    return state.longest as i32 - 1; // minus 1 for starting point
}

fn calc1() -> i32 {
    let handle = std::thread::Builder::new()
        .stack_size(512 * 1024 * 1024)
        .spawn(|| get_longest(get_valid_dirs1))
        .unwrap();
    return handle.join().unwrap();
}

fn get_valid_dirs2(_c: char) -> Vec<(i32, i32)> {
    vec![(1, 0), (-1, 0), (0, 1), (0, -1)]
}

fn calc2() -> i32 {
    let handle = std::thread::Builder::new()
        .stack_size(512 * 1024 * 1024)
        .spawn(|| get_longest(get_valid_dirs2))
        .unwrap();
    return handle.join().unwrap();
}

pub fn calc() {
    println!("{}", calc1());
    println!("{}", calc2());
}

const N: usize = 23;
const INPUT: &str = r#"#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#"#;
