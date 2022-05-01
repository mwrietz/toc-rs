use colored::Colorize;
use glob::glob;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // get list of files in cwd
    for entry in glob("*").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => find(&path),
            Err(e) => println!("{:?}", e),
        }
    }
}

fn find(path: &Path) {
    let p: String = (&path.display()).to_string();
    println!("{}", p.blue());
    let mut lines = Vec::new();
    read_file_to_vector(&path, &mut lines);

    let mut l_num: u32 = 0;
    for line in &lines {
        l_num += 1;
        if line.contains("fn ") {
            let mut l = line.clone();
            if l.ends_with("{") {
                l.pop();
            }
            println!("{:>5} : {}", l_num.to_string().red(), l.trim());
        }
        if line.contains("struct ") {
            let mut l = line.clone();
            if l.ends_with("{") {
                l.pop();
            }
            println!("{:>5} : {}", l_num.to_string().red(), l.trim().yellow());
        }
    }
    println!("");
}

fn read_file_to_vector(file_path: &Path, vector: &mut Vec<String>) {
    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(ip) = line {
                vector.push(ip);
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
