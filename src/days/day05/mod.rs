use super::Day;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Write;
use std::time::Instant;

pub fn part1(day: &mut Day) {
    let now = Instant::now();

    let mut rules: HashMap<u8, Vec<u8>> = HashMap::new();

    day.input.iter().take_while(|&x| x != "").for_each(|x| {
        if let Some((a, b)) = x.split_once("|") {
            rules
                .entry(b.parse().unwrap())
                .or_insert(Vec::new())
                .push(a.parse().unwrap());
        }
    });

    let updates: Vec<Vec<u8>> = day
        .input
        .iter()
        .skip(day.input.iter().position(|x| *x == "").unwrap() + 1)
        .map(|x| {
            x.split(",")
                .into_iter()
                .map(|n| n.parse::<u8>().unwrap())
                .collect()
        })
        .collect();

    let parse = now.elapsed();
    let now = Instant::now();

    let result = updates
        .iter()
        .filter(|&x| {
            !x.iter().enumerate().any(|(idx, n)| {
                x.iter().skip(idx + 1).any(|y| match rules.get(n) {
                    Some(v) => v.contains(y),
                    None => false,
                })
            })
        })
        .map(|x| x[x.len() / 2] as u32)
        .sum::<u32>();

    if !day.test {
        write!(
            day.part1,
            "{} ({:.2?} parse, {:.2?} impl)",
            result,
            parse,
            now.elapsed()
        )
        .unwrap();
    } else {
        write!(day.part1, "{}", result).unwrap();
    }
}

pub fn part2(day: &mut Day) {
    let now = Instant::now();

    let mut rules: HashMap<u8, Vec<u8>> = HashMap::new();

    day.input.iter().take_while(|&x| x != "").for_each(|x| {
        if let Some((a, b)) = x.split_once("|") {
            rules
                .entry(b.parse().unwrap())
                .or_insert(Vec::new())
                .push(a.parse().unwrap());
        }
    });

    let updates: Vec<Vec<u8>> = day
        .input
        .iter()
        .skip(day.input.iter().position(|x| *x == "").unwrap() + 1)
        .map(|x| {
            x.split(",")
                .into_iter()
                .map(|n| n.parse::<u8>().unwrap())
                .collect()
        })
        .collect();

    let parse = now.elapsed();
    let now = Instant::now();

    let result = updates
        .iter()
        .filter(|&x| {
            x.iter().enumerate().any(|(idx, n)| {
                x.iter().skip(idx + 1).any(|y| match rules.get(n) {
                    Some(v) => v.contains(y),
                    None => false,
                })
            })
        })
        .map(|x| {
            x.clone().sort_by(|a, b| match rules.get(b) {
                Some(previous_values) => {
                    if previous_values.iter().any(|cmp| cmp == a) {
                        // b.cmp(a)
                        Ordering::Less
                    } else {
                        // 0.cmp(&0)
                        Ordering::Equal
                    }
                }
                None => Ordering::Equal,
            });
            let mut cp = x.clone();
            cp.sort_by(|a, b| match rules.get(a) {
                Some(previous_values) => {
                    if previous_values.iter().any(|cmp| cmp == b) {
                        Ordering::Less
                    } else {
                        Ordering::Equal
                    }
                }
                None => Ordering::Equal,
            });
            cp[x.len() / 2] as u32
        })
        .sum::<u32>();

    if !day.test {
        write!(
            day.part2,
            "{} ({:.2?} parse, {:?} impl)",
            result,
            parse,
            now.elapsed()
        )
        .unwrap();
    } else {
        write!(day.part2, "{}", result).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::days::DayBuilder;

    #[test]
    fn live_test() {
        let mut day = DayBuilder::new(5).as_test().build();
        part1(&mut day);
        assert_eq!(day.part1, "143");
        part2(&mut day);
        assert_eq!(day.part2, "123");
    }
}
