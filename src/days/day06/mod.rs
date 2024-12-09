use super::Day;
use std::collections::HashSet;
use std::fmt::Write;
use std::ops::Range;
use std::time::Instant;

#[derive(Debug)]
struct Guard {
    x_bound: Range<isize>,
    y_bound: Range<isize>,
    pos: (usize, usize),
    direction: Direction,
    steps: HashSet<(usize, usize)>,
}

impl Guard {
    fn new(v: &Vec<String>) -> Self {
        let mut pos = (0, 0);
        v.iter().enumerate().for_each(|(x, s)| {
            if let Some(y) = s.find("^") {
                pos.0 = x as usize;
                pos.1 = y as usize;
            }
        });

        Guard {
            y_bound: 0 as isize..v.len() as isize,
            x_bound: 0 as isize..v[0].len() as isize,
            steps: HashSet::new(),
            pos: pos,
            direction: Direction::Up,
        }
    }
    fn change_direction(&mut self) {
        self.direction.next();
    }

    fn step(&mut self) -> Result<(), ()> {
        match self.direction {
            Direction::Up => {
                if self.y_bound.contains(&(self.pos.0 as isize - 1)) {
                    // println!("inbounds");
                    self.pos.0 -= 1;
                    self.steps.insert(self.pos);
                    return Ok(());
                }
            }
            Direction::Down => {
                if self.y_bound.contains(&(self.pos.0 as isize + 1)) {
                    // println!("inbounds");
                    self.pos.0 += 1;
                    self.steps.insert(self.pos);
                    return Ok(());
                }
            }
            Direction::Right => {
                if self.x_bound.contains(&(self.pos.1 as isize + 1)) {
                    // println!("inbounds");
                    self.pos.1 += 1;
                    self.steps.insert(self.pos);
                    return Ok(());
                }
            }
            Direction::Left => {
                if self.x_bound.contains(&(self.pos.1 as isize - 1)) {
                    // println!("inbounds");
                    self.pos.1 -= 1;
                    self.steps.insert(self.pos);
                    return Ok(());
                }
            }
        }
        // println!("outbounds");
        Err(())
    }

    fn locate_or_relocate(&mut self, c: char) {
        println!("We're placed in: [{}]", c);
        match c {
            '#' => match self.direction {
                Direction::Up => {
                    self.steps.remove(&self.pos);
                    self.pos.0 += 1;
                    println!("We're back into: [{:?}]", self.pos);
                    self.change_direction();
                }
                Direction::Down => {
                    self.steps.remove(&self.pos);
                    self.pos.0 -= 1;
                    println!("We're back into: [{:?}]", self.pos);
                    self.change_direction();
                }
                Direction::Right => {
                    self.steps.remove(&self.pos);
                    self.pos.1 -= 1;
                    println!("We're back into: [{:?}]", self.pos);
                    self.change_direction();
                }
                Direction::Left => {
                    self.steps.remove(&self.pos);
                    self.pos.1 += 1;
                    println!("We're back into: [{:?}]", self.pos);
                    self.change_direction();
                }
            },
            _ => {
                println!(
                    "We're into coordinates: [{:?}] - Steps: {}",
                    self.pos,
                    self.steps.len()
                );
            }
        }
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn next(&mut self) {
        *self = match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

pub fn part1(day: &mut Day) {
    let now = Instant::now();

    let mut guard = Guard::new(&day.input);

    let parse = now.elapsed();
    let now = Instant::now();

    while let Ok(()) = guard.step() {
        // println!("Situation after step: {:?}", guard);
        guard.locate_or_relocate(day.input[guard.pos.0].chars().nth(guard.pos.1).unwrap());
    }

    // println!(" FINAL => {:?}", guard);

    if !day.test {
        write!(
            day.part1,
            "{} ({:.2?} parse, {:.2?} impl)",
            guard.steps.len() + 1,
            parse,
            now.elapsed()
        )
        .unwrap();
    } else {
        write!(day.part1, "{}", guard.steps.len() + 1).unwrap();
    }
}

pub fn part2(day: &mut Day) {
    let now = Instant::now();

    let parse = now.elapsed();
    let now = Instant::now();

    // if !day.test {
    //     write!(
    //         day.part2,
    //         "{} ({:.2?} parse, {:?} impl)",
    //         result,
    //         parse,
    //         now.elapsed()
    //     )
    //     .unwrap();
    // } else {
    //     write!(day.part2, "{}", result).unwrap();
    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::days::DayBuilder;

    #[test]
    fn live_test() {
        let mut day = DayBuilder::new(6).as_test().build();
        part1(&mut day);
        assert_eq!(day.part1, "41");
        // part2(&mut day);
        // assert_eq!(day.part2, "123");
    }
}

//     match dir {
//         Direction::Up => {
//             while day.input[pos.0].chars().nth(pos.1).unwrap() != '#' {
//                 println!(
//                     "inner pos[{:?}{:?}] val[{}]",
//                     pos,
//                     dir,
//                     day.input[pos.0].chars().nth(pos.1).unwrap()
//                 );
//                 result += 1;
//                 if out_bounds(
//                     &mut (pos.0 - 1, pos.1),
//                     dir,
//                     day.input[pos.0],
//                     y_bound,
//                     x_bound,
//                 ) {
//                     break;
//                 }
//                 pos.0 -= 1;
//             }
//             if !out_bounds(pos, y_bound, x_bound) {
//                 dir.next();
//             } else {
//                 result += 1;
//                 break;
//             }
//         }
//         Direction::Right => {
//             while day.input[pos.0].chars().nth(pos.1).unwrap() != '#' {
//                 println!(
//                     "inner pos[{:?}{:?}] val[{}]",
//                     pos,
//                     dir,
//                     day.input[pos.0].chars().nth(pos.1).unwrap()
//                 );
//                 result += 1;
//                 if out_bounds((pos.0, pos.1 + 1), y_bound, x_bound) {
//                     break;
//                 }
//                 pos.1 += 1;
//             }
//             if !out_bounds(pos, y_bound, x_bound) {
//                 dir.next();
//             } else {
//                 result += 1;
//                 break;
//             }
//         }
//         Direction::Down => {
//             while day.input[pos.0].chars().nth(pos.1).unwrap() != '#' {
//                 println!(
//                     "inner pos[{:?}{:?}] val[{}]",
//                     pos,
//                     dir,
//                     day.input[pos.0].chars().nth(pos.1).unwrap()
//                 );
//                 result += 1;
//                 if out_bounds((pos.0 - 1, pos.1), y_bound, x_bound) {
//                     break;
//                 }
//                 pos.0 -= 1;
//             }
//             if !out_bounds(pos, y_bound, x_bound) {
//                 dir.next();
//             } else {
//                 result += 1;
//                 break;
//             }
//         }
//         Direction::Left => {
//             while day.input[pos.0].chars().nth(pos.1).unwrap() != '#' {
//                 println!(
//                     "inner pos[{:?}{:?}] val[{}]",
//                     pos,
//                     dir,
//                     day.input[pos.0].chars().nth(pos.1).unwrap()
//                 );
//                 result += 1;
//                 if out_bounds((pos.0, pos.1 - 1), y_bound, x_bound) {
//                     break;
//                 }
//                 pos.1 -= 1;
//             }
//             if !out_bounds(pos, y_bound, x_bound) {
//                 dir.next();
//             } else {
//                 result += 1;
//                 break;
//             }
//         }
//     }
// }

// fn out_bounds(
//     pos: &mut (usize, usize),
//     dir: &Direction,
//     line: &str,
//     y_bound: usize,
//     x_bound: usize,
// ) -> bool {
//     if pos.0 == 0 || pos.0 == y_bound || pos.1 == 0 || pos.1 == x_bound {
//         match dir {
//             Direction::Up => {
//                 if line.chars().nth(pos.1).unwrap() == '#' {
//                     pos.0 += 1;
//                     return false;
//                 }
//                 return true;
//             }
//             Direction::Right => {
//                 if line.chars().nth(pos.1).unwrap() == '#' {
//                     pos.1 -= 1;
//                     return false;
//                 }
//                 return true;
//             }
//             Direction::Down => {
//                 if line.chars().nth(pos.1).unwrap() == '#' {
//                     pos.0 -= 1;
//                     return false;
//                 }
//                 return true;
//             }
//             Direction::Left => {
//                 if line.chars().nth(pos.1).unwrap() == '#' {
//                     pos.1 -= 1;
//                     return false;
//                 }
//                 return true;
//             }
//         }
