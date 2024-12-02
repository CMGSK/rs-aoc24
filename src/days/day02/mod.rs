use super::Day;
use std::fmt::Write;
use std::time::Instant;

pub fn part1(day: &mut Day) {
    let now = Instant::now();
    let mut result: u32 = 0;

    for line in &day.input {
        let rep: Vec<u16> = line
            .split_whitespace()
            .map(|x| x.parse::<u16>().unwrap())
            .collect();

        if is_ordered_safe(&rep) {
            result += 1;
        }
    }

    if !day.test {
        write!(day.part1, "{} ({:.2?})", result, now.elapsed()).unwrap();
    } else {
        write!(day.part1, "{}", result).unwrap();
    }

    fn is_ordered_safe(v: &Vec<u16>) -> bool {
        let mut asc = true;
        let mut des = true;

        for n in 1..v.len() {
            if v[n] < v[n - 1] {
                asc = false;
            } else {
                des = false;
            }

            if (!asc && !des) || (v[n].abs_diff(v[n - 1]) > 3 || v[n].abs_diff(v[n - 1]) < 1) {
                return false;
            }
        }
        true
    }
}

pub fn part2(day: &mut Day) {
    let now = Instant::now();
    let mut result: u32 = 0;

    for line in &day.input {
        let rep: Vec<u16> = line
            .split_whitespace()
            .map(|x| x.parse::<u16>().unwrap())
            .collect();

        if is_ordered_safe(&rep, false) == 0 {
            result += 1;
        }
    }

    if !day.test {
        write!(day.part2, "{} ({:.2?})", result, now.elapsed()).unwrap();
    } else {
        write!(day.part2, "{}", result).unwrap();
    }

    fn is_ordered_safe(v: &Vec<u16>, corrupt: bool) -> usize {
        let mut asc = true;
        let mut des = true;

        for n in 1..v.len() {
            if v[n] < v[n - 1] {
                asc = false;
            } else {
                des = false;
            }

            if (!asc && !des) || (v[n].abs_diff(v[n - 1]) > 3 || v[n].abs_diff(v[n - 1]) < 1) {
                if corrupt {
                    return n;
                }
                let mut safe = v.clone();
                let recover = safe.remove(n);
                if is_ordered_safe(&safe, true) != 0 {
                    safe.insert(n, recover);
                    safe.remove(n - 1);
                    if is_ordered_safe(&safe, true) != 0 {
                        return n;
                    }
                }
                return 0;
            }
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::days::DayBuilder;

    #[test]
    fn live_test() {
        let mut day = DayBuilder::new(2).as_test().build();
        part1(&mut day);
        assert_eq!(day.part1, "2");
        part2(&mut day);
        assert_eq!(day.part2, "4");
    }
}
