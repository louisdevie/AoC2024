mod puzzle1;
mod puzzle1_part2;

pub fn solve(puzzle: &str) {
    match puzzle {
        "1" => println!("The answer is {}", puzzle1::solve()),
        "1+" => println!("The answer is {}", puzzle1_part2::solve()),
        _ => println!("Puzzle {} unknown.", puzzle),
    };
}
