use hashbrown::HashMap;
use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

fn parse(str: &str) -> HashMap<&str, HashSet<&str>> {
    let mut map = HashMap::new();
    for line in str.lines() {
        let mut parts = line.split(": ");
        let key = parts.next().unwrap();
        let values = parts.next().unwrap().split(' ').collect::<HashSet<_>>();
        map.entry(key)
            .or_insert(HashSet::new())
            .extend(values.clone().into_iter());
        for k in values.into_iter() {
            map.entry(k).or_insert(HashSet::new()).insert(key);
        }
    }
    return map;
}

fn calc1() -> i32 {
    let mut map: HashMap<&str, HashSet<&str>> = parse(INPUT);

    let mut longest_required_to_touch_everything = Vec::<(&str, usize)>::new();
    for starting_k in map.keys() {
        let mut queue = VecDeque::<(&str, usize)>::new();
        let mut seen = HashSet::<&str>::new();
        let mut longest = 0;
        queue.push_back((starting_k, 0));
        while let Some(k) = queue.pop_front() {
            longest = longest.max(k.1);
            seen.insert(k.0);
            for k2 in map.get(k.0).unwrap() {
                if seen.contains(k2) {
                    continue;
                }
                queue.push_back((k2, k.1 + 1));
            }
        }
        // println!("{}: {}", starting_k, longest)
        longest_required_to_touch_everything.push((starting_k, longest));
    }
    let to_disconnect = longest_required_to_touch_everything
        .iter()
        .sorted_by_key(|(_, n)| *n)
        .map(|(k, _)| *k)
        .take(6)
        .collect::<Vec<_>>();
    // for x in longests.iter().sorted_by_key(|(_, n)| *n).take(16) {
    //     println!("{:?}", x);
    // }

    for k1 in &to_disconnect {
        for k2 in &to_disconnect {
            map.get_mut(k1).unwrap().remove(k2);
            map.get_mut(k2).unwrap().remove(k1);
        }
    }

    let mut group_sizes = HashSet::<usize>::new();
    for starting_k in &to_disconnect {
        let mut queue = VecDeque::<&str>::new();
        let mut seen = HashSet::<&str>::new();
        queue.push_back(starting_k);
        while let Some(k) = queue.pop_front() {
            seen.insert(k);
            for k2 in map.get(k).unwrap() {
                if seen.contains(k2) {
                    continue;
                }
                queue.push_back(k2);
            }
        }
        group_sizes.insert(seen.len());
    }
    // println!("sizes {:?}", group_sizes);

    let mut group_sizes_iter = group_sizes.iter();
    return *group_sizes_iter.next().unwrap() as i32 * *group_sizes_iter.next().unwrap() as i32;
}

fn calc2() -> i32 {
    return 0;
}

pub fn calc() {
    println!("{}", calc1());
    println!("{}", calc2());
}

const INPUT: &str = r#"jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr"#;
