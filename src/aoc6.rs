fn calc1() -> i32 {
    let mut res = 1;
    for i in 0..TIMES.len() {
        let mut c = 0;
        let time = TIMES[i];
        for t in 0..time {
            let d = t * (time - t);
            if d > DISTS[i] {
                c += 1;
            }
        }
        res *= c;
    }
    return res;
}

fn calc2() -> i32 {
    let mut c = 0;
    for t in 0..TIME2 {
        let d = t * (TIME2 - t);
        if d > DIST2 {
            c += 1;
        }
    }
    return c;
}

pub fn calc() {
    println!("{}\n{}", calc1(), calc2());
}

const TIMES: [i64; 3] = [7, 15, 30];
const DISTS: [i64; 3] = [9, 40, 200];
const TIME2: i64 = 71530;
const DIST2: i64 = 940200;
