use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
fn main() -> std::io::Result<()> {
    let f: File = File::open("<filepath>")?;
    let reader: BufReader<File> = BufReader::new(f);

    let mut vec1: Vec<i32> = Vec::new();
    let mut vec2: Vec<i32> = Vec::new();

    for lines in reader.lines()  {
        //Gut the lines
        let fappy = lines.unwrap_or_default();
        let mut split_string= fappy.split_whitespace();
        vec1.push(
            split_string.next().unwrap().parse::<i32>().unwrap(),
        );
        vec2.push(
            split_string.next().unwrap().parse::<i32>().unwrap(),
        );
    }

    vec1.sort();
    vec2.sort();
    Ok(())
}
