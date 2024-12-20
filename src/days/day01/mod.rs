use super::Day;
use std::fmt::Write;
use std::time::Instant;

pub fn part1(day: &mut Day) {
    let now = Instant::now();
    let mut result: u32 = 0;

    let mut r: Vec<u32> = Vec::new();
    let mut l: Vec<u32> = Vec::new();
    for line in &day.input {
        let pair: Vec<u32> = line
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        r.push(pair[0]);
        l.push(pair[1]);
    }
    r.sort_unstable();
    l.sort_unstable();

    while let Some(rv) = r.pop() {
        let lv = l.pop().unwrap();
        result += rv.abs_diff(lv);
    }

    if !day.test {
        write!(day.part1, "{} ({:.2?})", result, now.elapsed()).unwrap();
    } else {
        write!(day.part1, "{}", result).unwrap();
    }
}

pub fn part2(day: &mut Day) {
    let now = Instant::now();
    let mut result: u32 = 0;

    let mut r: Vec<u32> = Vec::new();
    let mut l: Vec<u32> = Vec::new();
    for line in &day.input {
        let pair: Vec<u32> = line
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        r.push(pair[0]);
        l.push(pair[1]);
    }
    r.sort_unstable();
    l.sort_unstable();

    while let Some(rv) = r.pop() {
        result += rv * count_rep(&l, rv);
    }

    if !day.test {
        write!(day.part2, "{} ({:.2?})", result, now.elapsed()).unwrap();
    } else {
        write!(day.part2, "{}", result).unwrap();
    }

    fn count_rep(v: &Vec<u32>, t: u32) -> u32 {
        let mut freq: u32 = 0;
        for n in v {
            freq += (*n == t) as u32;
        }
        freq
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::days::DayBuilder;

    #[test]
    fn live_test() {
        let mut day = DayBuilder::new(1).as_test().build();
        part1(&mut day);
        assert_eq!(day.part1, "11");
        println!("Part 1 passed");
        part2(&mut day);
        assert_eq!(day.part2, "31");
    }
}
