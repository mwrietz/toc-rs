use glob::glob;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crossterm::style::Stylize;

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
        for (i, arg) in args.iter().enumerate() {
            if i > 0 {
                let p = Path::new(arg);
                find(p, &mut termstat);
            }
        }
    }

    gh_repo_status::check_version().expect("check_version error");
}

fn find(path: &Path, termstat: &mut tui_gen::TermStat) {
    let p = (path.display()).to_string();
    println!("{}", p.clone().dark_blue());
    termstat.line_check();

    if !p.ends_with(".rs") {
        println!("ignoring - not a rust source file...\n");
        termstat.line_check();
        return;
    }

    let mut lines = Vec::new();
    read_file_to_vector(path, &mut lines);

    let mut l_num: u32 = 0;

    for line in lines.iter_mut() {
        l_num += 1;

        if line.ends_with('{') {
            line.pop();
        }

        if line.contains("fn ") {
            println!("{} : {}", format!("{:>5}", l_num).dark_red(), line.trim_end());
        }
        if line.contains("struct ") {
            println!(
                "{} : {}",
                format!("{:>5}", l_num).dark_red(),
                line.trim_end().dark_yellow()
            );
        }
        if line.starts_with("use ") {
            println!(
                "{} : {}",
                format!("{:>5}", l_num).dark_red(),
                line.trim_end().dark_cyan()
            );
        }
        if line.starts_with("mod ") {
            println!(
                "{} : {}",
                format!("{:>5}", l_num).dark_red(),
                line.trim_end().dark_magenta()
            );
        }
        termstat.line_check();
    }
    println!();
    termstat.line_check();
}

fn read_file_to_vector(file_path: &Path, vector: &mut Vec<String>) {
    if let Ok(lines) = read_lines(file_path) {
        for line in lines.map_while(Result::ok) {
            vector.push(line);
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
