use super::Day;
use std::fmt::Write;
use std::time::Instant;
use std::vec;

const XMAS: [&[u8; 4]; 2] = [b"XMAS", b"SAMX"];
const X: u8 = b'X';
const A: u8 = b'A';

pub fn part1(day: &mut Day) {
    //Sue me for not messing with the input for part two
    let mut padding = day.input.clone();
    let now = Instant::now();

    padding.append(vec![String::from_utf8(vec![b'@'; padding[0].len()]).unwrap(); 3].as_mut());

    let result = padding
        .windows(4)
        .map(|x| find_patterns(x.to_vec()) as u64)
        .sum::<u64>();

    if !day.test {
        write!(day.part1, "{} ({:.2?})", result, now.elapsed()).unwrap();
    } else {
        write!(day.part1, "{}", result).unwrap();
    }

    fn find_patterns(map: Vec<String>) -> u32 {
        let cur = map[0].as_bytes();
        let fir = map[1].as_bytes();
        let sec = map[2].as_bytes();
        let thi = map[3].as_bytes();
        map[0]
            .chars()
            .into_iter()
            .enumerate()
            .fold(0, |mut acc, (i, c)| {
                if c != 'X' && c != 'S' {
                    return acc;
                }

                if XMAS.contains(&&[c as u8, fir[i], sec[i], thi[i]]) {
                    acc += 1;
                }
                if i < map[0].len() - 3 {
                    if XMAS.contains(&&[c as u8, fir[i + 1], sec[i + 2], thi[i + 3]]) {
                        acc += 1;
                    }
                    if XMAS.contains(&&[cur[i], cur[i + 1], cur[i + 2], cur[i + 3]]) {
                        acc += 1;
                    }
                }
                if i >= 3 {
                    if XMAS.contains(&&[c as u8, fir[i - 1], sec[i - 2], thi[i - 3]]) {
                        acc += 1;
                    }
                }
                acc
            }) as u32
    }
}

pub fn part2(day: &mut Day) {
    let now = Instant::now();
    let result = day
        .input
        .windows(3)
        .map(|x| find_patterns(x.to_vec()) as u64)
        .sum::<u64>();

    if !day.test {
        write!(day.part2, "{} ({:.2?})", result, now.elapsed()).unwrap();
    } else {
        write!(day.part2, "{}", result).unwrap();
    }

    fn find_patterns(map: Vec<String>) -> u32 {
        map[1]
            .chars()
            .into_iter()
            .enumerate()
            .fold(0, |acc, (i, c)| {
                if c != A as char || i == map[1].len() - 1 || i == 0 {
                    return acc;
                }
                let up = map[0].as_bytes();
                let dn = map[2].as_bytes();

                if (up[i - 1] != dn[i + 1] && up[i + 1] != dn[i - 1])
                    && (X != up[i - 1] && A != dn[i + 1])
                    && (A != up[i - 1] && X != dn[i + 1])
                    && (A != up[i + 1] && X != dn[i - 1])
                    && (X != up[i + 1] && A != dn[i - 1])
                {
                    return acc + 1;
                }
                acc
            }) as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::days::DayBuilder;

    #[test]
    fn live_test() {
        let mut day = DayBuilder::new(4).as_test().build();
        part1(&mut day);
        assert_eq!(day.part1, "18");
        part2(&mut day);
        assert_eq!(day.part2, "9");
    }
}
