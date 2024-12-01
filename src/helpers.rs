use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

pub fn read_data_lines<P>(file_name: P) -> io::Lines<io::BufReader<File>>
where
    P: AsRef<Path>,
{
    let file = File::open(Path::new("./data").join(file_name)).expect("Data file not found.");
    io::BufReader::new(file).lines()
}
