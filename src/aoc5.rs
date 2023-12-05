use std::cmp;

#[derive(Copy, Clone, Debug)]
struct Map {
    dest: i64,
    src: i64,
    len: i64,
}

#[derive(Debug)]
struct Input {
    seeds: Vec<i64>,
    map_chain: Vec<Vec<Map>>,
}

fn parse(str: &str) -> Input {
    let mut split = str.split("\n\n");
    let first = split.next().unwrap().trim();
    let seeds = first
        .split(" ")
        .skip(1)
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let map_chain = split
        .map(|l| {
            let lines: std::iter::Skip<std::str::Split<'_, &str>> = l.split("\n").skip(1);
            return lines
                .map(|s| {
                    let mut nums = s.split(" ").map(|s| s.parse::<i64>().unwrap());
                    return Map {
                        dest: nums.next().unwrap(),
                        src: nums.next().unwrap(),
                        len: nums.next().unwrap(),
                    };
                })
                .collect::<Vec<_>>();
        })
        .collect::<Vec<_>>();
    return Input { seeds, map_chain };
}

fn get_mapped(n: i64, maps: &Vec<Map>) -> i64 {
    for m in maps {
        if n >= m.src && n < (m.src + m.len) {
            return m.dest + (n - m.src);
        }
    }
    return n;
}

fn calc1() -> i64 {
    let input = parse(INPUT);
    let mut lowest = i64::MAX;
    for n in input.seeds {
        let mut final_n = n;
        // println!("start {}", final_n);
        for maps in &input.map_chain {
            final_n = get_mapped(final_n, maps);
            // println!("{}", final_n);
        }
        lowest = cmp::min(lowest, final_n);
    }
    return lowest;
}

fn calc2() -> i64 {
    // oops
    return 0;
}

pub fn calc() {
    println!("{}\n{}", calc1(), calc2());
}

const INPUT: &str = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;
