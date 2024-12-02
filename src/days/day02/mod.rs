use std::fmt::Write;
use super::Day;

pub fn part1(day: &mut Day) {
    let mut result: u32 = 0;

    for line in &day.input {}
}

pub fn part2(day: &mut Day) {
    let mut result: u32 = 0;

    for line in &day.input {}
}


#[cfg(test)]
mod tests{
    use crate::days::DayBuilder;
    use super::*;

    #[test]
    fn live_test(){
        let mut day = DayBuilder::new(2).as_test().build();
        part1(&mut day);
        assert_eq!(day.part1, "");
        part2(&mut day);
        assert_eq!(day.part2, "");
    }
}