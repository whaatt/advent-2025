use std::{
    fs::File,
    io::{self, BufRead, Read},
    path::Path,
};

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_string<P>(filename: P) -> io::Result<String>
where
    P: AsRef<Path>,
{
    let mut string = String::new();
    let file = File::open(filename)?;
    io::BufReader::new(file).read_to_string(&mut string)?;
    Ok(string)
}
