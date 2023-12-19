#[derive(Debug)]
struct Input {
    cards: [i64; 5],
    bid: i32,
}

fn val(c: char) -> i64 {
    return match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        _ => c as i64 - '0' as i64,
    };
}

fn parse(str: &str, val_func: fn(c: char) -> i64) -> Vec<Input> {
    let mut res = Vec::new();
    for line in str.split("\n") {
        let mut parts = line.split(" ");
        let mut chars = parts.next().unwrap().chars();
        let cards = [
            val_func(chars.next().unwrap()),
            val_func(chars.next().unwrap()),
            val_func(chars.next().unwrap()),
            val_func(chars.next().unwrap()),
            val_func(chars.next().unwrap()),
        ];
        let bid = parts.next().unwrap().parse::<i32>().unwrap();
        res.push(Input { cards, bid });
    }
    return res;
}

const KIND5: i64 = 600____000_000_000_000_000;
const KIND4: i64 = 500____000_000_000_000_000;
const FULLH: i64 = 400____000_000_000_000_000;
const KIND3: i64 = 300____000_000_000_000_000;
const PAIR2: i64 = 200____000_000_000_000_000;
const PAIR1: i64 = 100____000_000_000_000_000;

fn key(c: &Input) -> i64 {
    // rank
    let mut counts = [0_usize; 15];
    for i in 0..5 {
        counts[c.cards[i] as usize] += 1;
    }
    counts.sort();
    let last = counts[14];
    let last2 = counts[13];
    let mut res = match (last, last2) {
        (5, _) => KIND5,
        (4, _) => KIND4,
        (3, 2) => FULLH,
        (3, _) => KIND3,
        (2, 2) => PAIR2,
        (2, _) => PAIR1,
        _ => 0,
    };
    // tiebreaker
    let mut exp: i64 = 1;
    for i in 0..5 {
        res += c.cards[4 - i] * exp;
        exp *= 1000;
    }
    return res;
}

fn calc1() -> i32 {
    let mut input = parse(INPUT, val);
    input.sort_by_cached_key(key);
    let sum = input
        .iter()
        .enumerate()
        .map(|(i, c)| c.bid * (i as i32 + 1))
        .sum::<i32>();
    return sum;
}

fn val2(c: char) -> i64 {
    return match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 1,
        'T' => 10,
        _ => c as i64 - '0' as i64,
    };
}

fn key2(c: &Input) -> i64 {
    // rank
    let mut counts = [0_usize; 15];
    let mut jokers = 0;
    for i in 0..5 {
        if c.cards[i] == 1 {
            jokers += 1;
        } else {
            counts[c.cards[i] as usize] += 1;
        }
    }
    counts.sort();
    let last = counts[14];
    let last2 = counts[13];
    let mut res = match (last, last2, jokers) {
        (5, _, _) => KIND5,
        (4, _, 1) => KIND5,
        (3, _, 2) => KIND5,
        (2, _, 3) => KIND5,
        (1, _, 4) => KIND5,
        (0, _, 5) => KIND5,
        (4, _, _) => KIND4,
        (3, _, 1) => KIND4,
        (2, _, 2) => KIND4,
        (1, _, 3) => KIND4,
        (0, _, 4) => KIND4,
        (3, 2, _) => FULLH,
        (3, 1, 1) => FULLH,
        (3, 0, 2) => FULLH,
        (2, 2, 1) => FULLH,
        (2, 1, 2) => FULLH,
        (2, 0, 3) => FULLH,
        (1, 0, 4) => FULLH,
        (3, _, _) => KIND3,
        (2, _, 1) => KIND3,
        (1, _, 2) => KIND3,
        (0, _, 3) => KIND3,
        (2, 2, _) => PAIR2,
        (2, 1, 1) => PAIR2,
        (2, 0, 2) => PAIR2,
        (1, 1, 2) => PAIR2,
        (2, _, _) => PAIR1,
        (1, _, 1) => PAIR1,
        _ => 0,
    };
    // tiebreaker
    let mut exp: i64 = 1;
    for i in 0..5 {
        res += c.cards[4 - i] * exp;
        exp *= 1000;
    }
    return res;
}

fn calc2() -> i32 {
    let mut input = parse(INPUT, val2);
    input.sort_by_cached_key(key2);
    for i in &input {
        println!("{:018} - {:?}", key2(&i), i);
    }
    let sum = input
        .iter()
        .enumerate()
        .map(|(i, c)| c.bid * (i as i32 + 1))
        .sum::<i32>();
    return sum;
}

pub fn calc() {
    println!("{}\n{}", calc1(), calc2());
}

const INPUT: &str = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;
