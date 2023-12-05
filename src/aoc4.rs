#[derive(Debug)]
struct Card {
    wins: Vec<i64>,
    mine: Vec<i64>,
}

type Input = Vec<Card>;

fn parse(str: &str) -> Input {
    let mut cards = Vec::<Card>::new();
    for line in str.split("\n") {
        if let [first, second] = line.split(" | ").collect::<Vec<_>>()[..] {
            let first2 = first.split(": ").skip(1).next().unwrap();
            let wins = first2
                .split(" ")
                .filter_map(|s| s.parse::<i64>().ok())
                .collect::<Vec<_>>();
            let mine = second
                .split(" ")
                .filter_map(|s| s.parse::<i64>().ok())
                .collect::<Vec<_>>();
            cards.push(Card { wins, mine });
        }
    }
    return cards;
}

fn calc1() -> i64 {
    let input = parse(INPUT);
    let mut total_score = 0;
    for c in input {
        let mut score = 1;
        for m in c.mine {
            if c.wins.contains(&m) {
                score *= 2;
            }
        }
        score /= 2; // becomes 0 if score was 1
                    // println!("score += {}", score);
        total_score += score;
    }
    return total_score;
}

fn calc2() -> i64 {
    let input = parse(INPUT);
    let mut copies = vec![1_u64; input.len()];
    for (i, c) in input.iter().enumerate() {
        let mut ii = 0;
        for m in &c.mine {
            if c.wins.contains(m) {
                ii += 1;
                copies[i + ii] += copies[i] as u64;
                // println!("{} added {} of card {}", i, copies[i], i + ii);
            }
        }
    }
    return copies.iter().sum::<u64>() as i64;
}

pub fn calc() {
    println!("{}\n{}", calc1(), calc2());
}

const INPUT: &str = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;
