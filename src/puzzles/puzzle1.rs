use crate::helpers::read_data_lines;
use std::io;

pub fn solve() -> i32 {
    let (mut left, mut right): (Vec<i32>, Vec<i32>) =
        read_data_lines("puzzle1.txt").map(split_line).unzip();
    left.sort();
    right.sort();
    left.iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum()
}

pub(crate) fn split_line(line: io::Result<String>) -> (i32, i32) {
    let mut nums = line
        .as_ref()
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse::<i32>().unwrap());
    let num_left = nums.next().unwrap();
    let num_right = nums.next().unwrap();
    (num_left, num_right)
}
