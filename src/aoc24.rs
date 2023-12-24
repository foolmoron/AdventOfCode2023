use regex::Regex;
use std::ops::Add;
use std::ops::Mul;
use std::ops::Sub;
use vecmath::mat3_det;

#[derive(PartialEq, Debug, Clone, Copy)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
struct PosVel {
    pos: Vec3,
    vel: Vec3,
}

fn parse(str: &str) -> Vec<PosVel> {
    let re = Regex::new(r"(\d+),\s+(\d+),\s+(\d+)\s+@\s+(-?\d+),\s+(-?\d+),\s+(-?\d+)").unwrap();
    return str
        .split("\n")
        .map(|line| re.captures(line).unwrap())
        .map(|caps| PosVel {
            pos: Vec3 {
                x: caps[1].parse().unwrap(),
                y: caps[2].parse().unwrap(),
                z: caps[3].parse().unwrap(),
            },
            vel: Vec3 {
                x: caps[4].parse().unwrap(),
                y: caps[5].parse().unwrap(),
                z: caps[6].parse().unwrap(),
            },
        })
        .collect();
}

struct TU {
    t: f64,
    u: f64,
}
fn calc_intersection_2d(a1: Vec3, a2: Vec3, b1: Vec3, b2: Vec3) -> TU {
    let t = ((a1.x - b1.x) * (b1.y - b2.y) - (a1.y - b1.y) * (b1.x - b2.x))
        / ((a1.x - a2.x) * (b1.y - b2.y) - (a1.y - a2.y) * (b1.x - b2.x));
    let u = ((a1.x - b1.x) * (a1.y - a2.y) - (a1.y - b1.y) * (a1.x - a2.x))
        / ((a1.x - a2.x) * (b1.y - b2.y) - (a1.y - a2.y) * (b1.x - b2.x));
    return TU { t, u };
}

fn calc1() -> i32 {
    let input = parse(INPUT);
    let mut collisions = 0;
    for i in 0..input.len() {
        for j in (i + 1)..input.len() {
            let a = input[i];
            let b = input[j];
            let tu = calc_intersection_2d(a.pos, a.pos + a.vel, b.pos, b.pos + b.vel);
            if tu.t < 0.0 || tu.u < 0.0 {
                // past
                continue;
            }
            let point = a.pos + a.vel * tu.t;
            if point.x < RANGE.0 || point.x > RANGE.1 || point.y < RANGE.0 || point.y > RANGE.1 {
                // out of area
                continue;
            }
            collisions += 1;
        }
    }
    return collisions;
}

fn does_intersect_3d(a1: Vec3, a2: Vec3, b1: Vec3, b2: Vec3) -> bool {
    let first = a1 - a2;
    let second = a2 - b1;
    let third = b1 - b2;
    let tetrahedron_volume = mat3_det([
        [first.x, first.y, first.z],
        [second.x, second.y, second.z],
        [third.x, third.y, third.z],
    ]);
    return tetrahedron_volume < 0.1;
}

fn calc2() -> i64 {
    let input = parse(INPUT);
    let mut n: i64 = 0;
    let mut sum = 0;
    for x in -999..=999 {
        for y in -999..=999 {
            for z in -999..=999 {
                let shift = Vec3 {
                    x: x as f64,
                    y: y as f64,
                    z: z as f64,
                };
                n += 1;
                if n % 100_000_000 == 0 {
                    println!("*{}", n);
                }
                let mut valid = true;
                for i in 0..10 {
                    if !does_intersect_3d(
                        input[i].pos,
                        input[i].pos + input[i].vel - shift,
                        input[i + 1].pos,
                        input[i + 1].pos + input[i + 1].vel - shift,
                    ) {
                        valid = false;
                        break;
                    }
                }
                if !valid {
                    continue;
                }
                let tu = calc_intersection_2d(
                    input[0].pos,
                    input[0].pos + input[0].vel - shift,
                    input[1].pos,
                    input[1].pos + input[1].vel - shift,
                );
                let point = input[0].pos + (input[0].vel - shift) * tu.t;
                sum = point.x.round() as i64 + point.y.round() as i64 + point.z.round() as i64;
                println!("candidate - {:?} - {:?} - {}", shift, point, sum);
            }
        }
    }
    return sum;
}

pub fn calc() {
    println!("{}", calc1());
    println!("{}", calc2());
}

const RANGE: (f64, f64) = (7.0, 27.0);
const INPUT: &str = r#"19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3"#;
