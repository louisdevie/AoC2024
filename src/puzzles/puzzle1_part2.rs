use crate::helpers::read_data_lines;
use crate::puzzles::puzzle1;

pub fn solve() -> i32 {
    let (mut left, mut right): (Vec<i32>, Vec<i32>) = read_data_lines("puzzle1.txt")
        .map(puzzle1::split_line)
        .unzip();

    left.sort();
    right.sort();

    let mut l_idx = 0;
    let mut r_idx = 0;
    let mut sum = 0;

    while l_idx < left.len() && r_idx < right.len() {
        let l = left[l_idx];
        let r = right[r_idx];

        // move the cursors until the numbers match
        if r < l {
            r_idx += 1;
        } else if r > l {
            l_idx += 1;
        } else {
            // count the number of times the same number appears on the left
            let mut l2 = Some(&l);
            let mut ln = 0;
            while l2 == Some(&l) {
                l_idx += 1;
                ln += 1;
                l2 = left.get(l_idx)
            }

            // count the number of times the same number appears on the right
            let mut r2 = Some(&r);
            let mut rn = 0;
            while r2 == Some(&r) {
                r_idx += 1;
                rn += 1;
                r2 = right.get(r_idx)
            }

            // add the score
            sum += l * ln * rn;
        }
    }
    sum
}
