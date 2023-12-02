use regex::Regex;

fn get_calib(line: &str) -> i32 {
    let re = Regex::new(r"\d").unwrap();
    let finds = re.find_iter(line).collect::<Vec<_>>();
    let first = finds.first().unwrap().as_str().parse::<i32>().unwrap();
    let last = finds.last().unwrap().as_str().parse::<i32>().unwrap();
    return first * 10 + last;
}

fn get_calib2(line: &str) -> i32 {
    let re = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine|\d)").unwrap();
    let first = str_to_num(re.find(line).unwrap().as_str());
    let line_rev = line.chars().rev().collect::<String>();
    let re_rev = Regex::new(r"(\d|enin|thgie|neves|xis|evif|ruof|eerht|owt|eno)").unwrap();
    let last = str_to_num(
        &re_rev
            .find(&line_rev)
            .unwrap()
            .as_str()
            .chars()
            .rev()
            .collect::<String>(),
    );
    return first * 10 + last;
}

fn str_to_num(str: &str) -> i32 {
    return match str {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => str.parse::<i32>().unwrap(),
    };
}

fn calc1() -> i32 {
    return INPUT1.split("\n").map(get_calib).sum();
}

fn calc2() -> i32 {
    return INPUT2.split("\n").map(get_calib2).sum();
}

pub fn calc() {
    println!("{}\n{}", calc1(), calc2());
}

const INPUT1: &str = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;

const INPUT2: &str = r#"threeight"#;
