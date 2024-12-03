use super::Day;
use std::fmt::Write;
use std::time::Instant;

pub fn part1(day: &mut Day) {
    let now = Instant::now();

    let prefix = "mul(";
    let suffix = ")";

    let result: u64 = find_mul(&day.input[0], prefix, suffix)
        .iter()
        .map(|x| x[0] as u64 * x[1] as u64)
        .sum();

    if !day.test {
        write!(day.part1, "{} ({:.2?})", result, now.elapsed()).unwrap();
    } else {
        write!(day.part1, "{}", result).unwrap();
    }

    fn find_mul(s: &str, pre: &str, suf: &str) -> Vec<[i16; 2]> {
        let mut l = Vec::new();

        let mut ptr = 0;
        while let Some(p_pos) = s[ptr..].find(pre) {
            let p_start = ptr + p_pos;
            let avd = p_start + pre.len();

            if let Some(s_pos) = s[avd..].find(suf) {
                let s_end = avd + s_pos + suf.len();
                ptr = avd;
                if p_start.abs_diff(s_end) > pre.len() + suf.len() + 7 {
                    continue;
                }
                let it: String = s[p_start..s_end]
                    .chars()
                    .filter(|x| x.is_numeric() || *x == ',')
                    .collect();
                if let Some(r) = it
                    .split(",")
                    .filter_map(|x| x.parse::<i16>().ok())
                    .try_fold([0, 0], |mut acc, n| {
                        if (acc[0] != 0 && acc[1] != 0) || n == 0 {
                            return None;
                        }
                        if acc[0] == 0 {
                            acc[0] = n;
                        } else if acc[1] == 0 {
                            acc[1] = n;
                        }
                        Some(acc)
                    })
                {
                    l.push(r);
                }
            } else {
                break;
            }
        }
        l
    }
}

pub fn part2(day: &mut Day) {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::days::DayBuilder;

    #[test]
    fn live_test() {
        let mut day = DayBuilder::new(3).as_test().build();
        part1(&mut day);
        assert_eq!(day.part1, "161");
        // part2(&mut day);
        assert_eq!(day.part2, "");
    }
}
