use regex::Regex;

fn valid_id(line: &str) -> Option<i32> {
    let re_id = Regex::new(r"Game (\d+):").unwrap();
    let re_red = Regex::new(r"(\d+) red").unwrap();
    let re_green = Regex::new(r"(\d+) green").unwrap();
    let re_blue = Regex::new(r"(\d+) blue").unwrap();
    let max_red = re_red
        .captures_iter(line)
        .map(|c| c.get(1).unwrap().as_str().parse::<i32>().unwrap())
        .max();
    let max_green = re_green
        .captures_iter(line)
        .map(|c| c.get(1).unwrap().as_str().parse::<i32>().unwrap())
        .max();
    let max_blue = re_blue
        .captures_iter(line)
        .map(|c| c.get(1).unwrap().as_str().parse::<i32>().unwrap())
        .max();
    // println!("{:?} {:?} {:?}", max_red, max_green, max_blue);
    if (max_red.is_some() && max_red.unwrap() > 12)
        || (max_green.is_some() && max_green.unwrap() > 13)
        || (max_blue.is_some() && max_blue.unwrap() > 14)
    {
        return None;
    }
    return Some(
        re_id
            .captures(line)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse::<i32>()
            .unwrap(),
    );
}

fn calc1() -> i32 {
    return INPUT.split("\n").filter_map(valid_id).sum();
}

fn power(line: &str) -> i32 {
    let re_red = Regex::new(r"(\d+) red").unwrap();
    let re_green = Regex::new(r"(\d+) green").unwrap();
    let re_blue = Regex::new(r"(\d+) blue").unwrap();
    let max_red = re_red
        .captures_iter(line)
        .map(|c| c.get(1).unwrap().as_str().parse::<i32>().unwrap())
        .max();
    let max_green = re_green
        .captures_iter(line)
        .map(|c| c.get(1).unwrap().as_str().parse::<i32>().unwrap())
        .max();
    let max_blue = re_blue
        .captures_iter(line)
        .map(|c| c.get(1).unwrap().as_str().parse::<i32>().unwrap())
        .max();
    // println!("{:?} {:?} {:?}", max_red, max_green, max_blue);
    return max_red.unwrap_or(1) * max_green.unwrap_or(1) * max_blue.unwrap_or(1);
}

fn calc2() -> i32 {
    return INPUT.split("\n").map(power).sum();
}

pub fn calc() {
    println!("{}\n{}", calc1(), calc2());
}

const INPUT: &str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
