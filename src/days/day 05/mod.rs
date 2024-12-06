pub fn part1(day: &mut Day) {
    let now = Instant::now();

    if !day.test {
        write!(day.part1, "{} ({:.2?})", result, now.elapsed()).unwrap();
    } else {
        write!(day.part1, "{}", result).unwrap();
    }
}

pub fn part2(day: &mut Day) {
    let now = Instant::now();

    if !day.test {
        write!(day.part2, "{} ({:.2?})", result, now.elapsed()).unwrap();
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
        let mut day = DayBuilder::new(2).as_test().build();
        part1(&mut day);
        assert_eq!(day.part1, "143");
        // part2(&mut day);
        // assert_eq!(day.part2, "4");
    }
}
