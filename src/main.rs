use colored::Colorize;
use glob::glob;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

mod gh_repo_status;
mod tui_gen;

fn main() {
    // check for commandline args
    let args: Vec<String> = env::args().collect();

    let mut termstat = tui_gen::TermStat::default();

    tui_gen::cls();
    println!(
        "{}: v{}\n",
        tui_gen::get_prog_name(),
        env!("CARGO_PKG_VERSION")
    );

    if args.len() < 2 {
        // get list of files in cwd
        for entry in glob("**/*").expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => find(&path, &mut termstat),
                Err(e) => println!("{:?}", e),
            }
        }
    } else {
        // show only files included as arguments
        let mut i = 0;
        for arg in &args {
            if i > 0 {
                let p = Path::new(arg);
                find(p, &mut termstat);
            }
            i += 1;
        }
    }

    gh_repo_status::check_version().expect("check_version error");
}

fn find(path: &Path, termstat: &mut tui_gen::TermStat) {
    let p: String = (&path.display()).to_string();
    println!("{}", p.blue());
    termstat.line_check();

    if !p.ends_with(".rs") {
        println!("ignoring - not a rust source file...\n");
        termstat.line_check();
        return;
    }

    let mut lines = Vec::new();
    read_file_to_vector(&path, &mut lines);

    let mut l_num: u32 = 0;
    for line in &lines {
        l_num += 1;

        let mut l = line.clone();
        if l.ends_with("{") {
            l.pop();
        }

        if line.contains("fn ") {
            println!("{:>5} : {}", l_num.to_string().red(), l.trim_end());
        }
        if line.contains("struct ") {
            println!("{:>5} : {}", l_num.to_string().red(), l.trim_end().yellow());
        }
        if line.starts_with("use ") {
            println!("{:>5} : {}", l_num.to_string().red(), l.trim_end().cyan());
        }
        if line.starts_with("mod ") {
            println!(
                "{:>5} : {}",
                l_num.to_string().red(),
                l.trim_end().magenta()
            );
        }
        termstat.line_check();
    }
    println!("");
    termstat.line_check();
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
