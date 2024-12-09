use days::*;

mod days;

macro_rules! exec_advent {
    ($($day_mod:ident, $day_num:expr), *) => {
        $(
            {
                let mut test = DayBuilder::new($day_num).as_test().build();
                $day_mod::part1(&mut test);
                $day_mod::part2(&mut test);
                println!("-----------------Day{:?}------------------", $day_num);
                println!("Tests: {:?} | {:?}", test.part1, test.part2);
                let mut day = DayBuilder::new($day_num).build();
                $day_mod::part1(&mut day);
                $day_mod::part2(&mut day);
                println!("Part 1: {:?}", day.part1);
                println!("Part 2: {:?}", day.part2);
                println!("---------------------------------------");
                println!();
            }
        )*
    };
}

fn main() {
    #[rustfmt::skip]
    exec_advent!(
        day01, 1,
        day02, 2,
        day03, 3,
        day04, 4,
        day05, 5,
        day06, 6
    );
}
