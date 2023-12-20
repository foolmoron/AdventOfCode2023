use std::collections::HashMap;

#[derive(Debug)]
struct Input {
    steps: Vec<usize>,
    starts: Vec<String>,
    node_map: HashMap<String, [String; 2]>,
}

fn parse(str: &str) -> Input {
    let re = regex::Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();
    let mut splits = str.split("\n\n");
    let steps = splits
        .next()
        .unwrap()
        .chars()
        .map(|c| if c == 'L' { 0 } else { 1 })
        .collect::<Vec<_>>();
    let node_map = splits
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            let key = caps.get(1).unwrap().as_str().to_string();
            let value1 = caps.get(2).unwrap().as_str().to_string();
            let value2 = caps.get(3).unwrap().as_str().to_string();
            return (key, [value1, value2]);
        })
        .collect::<HashMap<_, _>>();
    let starts = node_map
        .keys()
        .cloned()
        .filter(|key| key.ends_with("A"))
        .collect::<Vec<_>>();
    return Input {
        steps,
        node_map,
        starts,
    };
}

fn calc1() -> i32 {
    let input = parse(INPUT);
    let mut curr = "AAA";
    let mut count = 0;
    let mut i = 0;
    while curr != "ZZZ" {
        curr = &input.node_map[curr][input.steps[i]];
        i = (i + 1) % input.steps.len();
        count += 1;
    }
    return count;
}

fn calc2() -> i128 {
    let input = parse(INPUT);
    let step_count = input.steps.len() as i128;
    let mut curr = input.starts;
    let mut count = 0;
    let mut i = 0;
    let mut frequencies = vec![0; curr.len()];
    while frequencies.iter().any(|f| *f == 0) {
        count += 1;
        for n in 0..curr.len() {
            curr[n] = input.node_map[&curr[n]][input.steps[i]].clone();
            if curr[n].ends_with("Z") {
                frequencies[n] = count;
            }
        }
        i = (i + 1) % input.steps.len();
    }
    // assume freqs are mutually prime multiples of step count
    let beat_multiple = frequencies.iter().fold(1, |acc, x| acc * (x / step_count));
    return beat_multiple * step_count;
}

pub fn calc() {
    println!("{}\n{}", calc1(), calc2());
}

const INPUT: &str = r#""#;
