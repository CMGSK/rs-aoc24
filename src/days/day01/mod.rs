use super::Day;

pub fn part1(day: &mut Day) {

}

pub fn part2(day: &mut Day) {

}

#[cfg(test)]
mod tests{
    use crate::days::DayBuilder;
    use super::*;

    #[test]
    fn live_test(){
        let mut day = DayBuilder::new(1).as_test().build();
        part1(&mut day);
        assert_eq!(day.part1, "value");
        part2(&mut day);
        assert_eq!(day.part2, "value");
    }
}