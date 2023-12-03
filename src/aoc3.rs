use regex::Regex;

#[derive(Copy, Clone, Debug)]
struct Int2 {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct NumRect {
    num: i32,
    pos: Int2,
    size: Int2,
}
impl NumRect {
    fn contains(&self, point: &Int2) -> bool {
        return point.x >= self.pos.x
            && point.x < (self.pos.x + self.size.x)
            && point.y >= self.pos.y
            && point.y < (self.pos.y + self.size.y);
    }
}

#[derive(Debug)]
struct Input {
    nums: Vec<NumRect>,
    symbols: Vec<Int2>,
}

fn parse(str: &str) -> Input {
    let re_num: Regex = Regex::new(r"\d+").unwrap();
    let re_symbol: Regex = Regex::new(r"[^\d\.]").unwrap();

    let mut nums = Vec::<NumRect>::new();
    let mut symbols = Vec::<Int2>::new();
    for (i, line) in str.split_whitespace().enumerate() {
        for m in re_num.find_iter(line) {
            nums.push(NumRect {
                num: m.as_str().parse::<i32>().unwrap(),
                pos: Int2 {
                    x: m.start() as i32 - 1,
                    y: i as i32 - 1,
                },
                size: Int2 {
                    x: m.as_str().len() as i32 + 2,
                    y: 3,
                },
            })
        }
        for m in re_symbol.find_iter(line) {
            symbols.push(Int2 {
                x: m.start() as i32,
                y: i as i32,
            });
        }
    }
    return Input { nums, symbols };
}

fn calc1() -> i32 {
    let input = parse(INPUT);
    let valid_nums = input.nums.iter().filter_map(|n| {
        if input.symbols.iter().any(|s| n.contains(s)) {
            return Some(n.num);
        } else {
            return None;
        }
    });
    return valid_nums.sum();
}

fn calc2() -> i32 {
    let input = parse(INPUT);
    let gears = input.symbols.iter().filter_map(|s| {
        let nearby_nums = input
            .nums
            .iter()
            .filter(|n| n.contains(s))
            .collect::<Vec<_>>();
        if nearby_nums.len() == 2 {
            return Some((s, nearby_nums[0].num, nearby_nums[1].num));
        } else {
            return None;
        }
    });
    let gear_ratios = gears.map(|g| g.1 * g.2);
    return gear_ratios.sum();
}

pub fn calc() {
    println!("{}\n{}", calc1(), calc2());
}

const INPUT: &str = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;
