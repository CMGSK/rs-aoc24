use std::fs::read_to_string;
pub mod day01;
pub mod day02;
pub mod day03;

#[derive(Debug)]
pub struct Day {
    pub input: Vec<String>,
    pub part1: String,
    pub part2: String,
    pub test: bool,
}

pub struct DayBuilder {
    pub number: usize,
    pub input: Vec<String>,
    pub part1: String,
    pub part2: String,
    pub test: bool,
}

impl DayBuilder {
    pub fn new(day: usize) -> Self {
        Self {
            number: day,
            input: vec![],
            part1: String::new(),
            part2: String::new(),
            test: false,
        }
    }

    pub fn as_test(mut self) -> Self {
        self.test = true;
        self
    }

    pub fn get_input(&mut self) -> Vec<String> {
        if self.input.is_empty() {
            let path;
            if self.test {
                path = format!("src/days/day{:0>2}/test.txt", self.number);
            } else {
                path = format!("src/days/day{:0>2}/input.txt", self.number);
            }
            let contents = match read_to_string(path) {
                Ok(r) => r,
                Err(e) => panic!("Problem opening the file: {:?}", e),
            };
            self.input = contents.lines().map(String::from).collect();
        }
        self.input.clone()
    }

    pub fn build(mut self) -> Day {
        self.get_input();
        Day {
            input: self.input,
            part1: self.part1,
            part2: self.part2,
            test: self.test,
        }
    }
}
