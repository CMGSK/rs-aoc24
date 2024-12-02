use days::*;

mod days;

macro_rules! exec_advent {
    ($($day_mod:ident, $day_num:expr), *) => {
        $(
            {
                let mut test = DayBuilder::new($day_num).as_test().build();
                $day_mod::part1(&mut test);
                $day_mod::part2(&mut test);
                println!("Part 1 test --> {:?}", test.part1);
                println!("Part 2 test --> {:?}", test.part2);
                let mut day = DayBuilder::new($day_num).build();
                $day_mod::part1(&mut day);
                $day_mod::part2(&mut day);
                println!("{:?}", day.part1);
                println!("{:?}", day.part2);
            }
        )* 
    };
}

fn main() {
    exec_advent!(
        day01, 1
    );
}