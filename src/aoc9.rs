fn parse(str: &str) -> Vec<[[i32; N]; N]> {
    let ret = str
        .split("\n")
        .map(|line| {
            let nums = line
                .split(' ')
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            let mut arr = [[0; N]; N];
            for j in 0..N {
                arr[0][j] = nums[j];
            }
            return arr;
        })
        .collect::<Vec<_>>();
    return ret;
}

fn calc1() -> i32 {
    let mut sum = 0;
    let lines = parse(INPUT);
    for mut line in lines {
        for i in 1..N {
            for j in 1..=(N - i) {
                line[i][j - 1] = line[i - 1][j] - line[i - 1][j - 1];
            }
            let mut all_zero = true;
            for j in 0..=(N - i) {
                if line[i][j] != 0 {
                    all_zero = false;
                    break;
                }
            }
            if all_zero {
                for n in 0..=i {
                    sum += line[n][N - n - 1];
                }
                break;
            }
        }
    }
    return sum;
}

fn calc2() -> i32 {
    let mut sum = 0;
    let lines = parse(INPUT);
    for mut line in lines {
        for i in 1..N {
            for j in 1..=(N - i) {
                line[i][j - 1] = line[i - 1][j] - line[i - 1][j - 1];
            }
            let mut all_zero = true;
            for j in 0..=(N - i) {
                if line[i][j] != 0 {
                    all_zero = false;
                    break;
                }
            }
            if all_zero {
                let mut flip = 1;
                for n in 0..=i {
                    sum += line[n][0] * flip;
                    flip *= -1;
                }
                break;
            }
        }
    }
    return sum;
}

pub fn calc() {
    println!("{}\n{}", calc1(), calc2());
}

const N: usize = 6;
const INPUT: &str = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#;
