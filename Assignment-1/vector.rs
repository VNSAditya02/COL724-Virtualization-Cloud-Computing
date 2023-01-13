use std::env;

pub struct Vector {}

#[derive(Debug, PartialEq, Eq)]
pub struct Interval {
    pub start: i32,
    pub end: i32,
}

impl Interval {
    #[inline]
    pub fn new(start: i32, end: i32) -> Self {
        Interval { start, end }
    }
}

impl Vector {
    pub fn overlap(intervals: Vec<Interval>) -> Vec<Interval> {
        let mut temp: Vec<Interval> = Vec::new();
        let mut output: Vec<Interval> = Vec::new();
        for i in &intervals {
            temp.push(Interval::new(i.start, i.end));
        }
        temp.sort_by(|a, b| a.start.cmp(&b.start));
        let mut idx: usize = 0;
        for i in &temp {
            if idx == 0 {
                output.push(Interval::new(i.start, i.end));
                idx += 1;
            }
            else {
                let prev = output[idx - 1].end;
                if prev <= i.start {
                    output.push(Interval::new(i.start, i.end));
                    idx += 1;
                }
                else {
                    output[idx - 1].end = i.end
                }
            }
        }
        return output;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let intervals = &args[1..];

    let mut input: Vec<Interval> = Vec::new();
    let mut temp: i32 = 0;

    for (pos, e) in intervals.iter().enumerate() {
        if let 1=pos%2{
            input.push(Interval::new(temp, e.parse::<i32>().unwrap()))
        }
        else {
            temp = e.parse::<i32>().unwrap();
        }
    }

    println!("{:?}", Vector::overlap(input));
}