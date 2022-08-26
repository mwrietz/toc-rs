use colored::Colorize;
use glob::glob;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

mod tui_gen;
mod gh_repo_status;

struct TermStat {
    line_count: usize,
    height: usize,
}

impl Default for TermStat {
    fn default() -> TermStat {
        TermStat {
            line_count: 0,
            height: 0,
        }
    }
}

impl TermStat {
    fn line_check(&mut self) {
        self.line_count += 1;
        if self.line_count > (self.height - 8) {
            tui_gen::pause();
            self.line_count = 0;
            tui_gen::cls();
            tui_gen::cmove(0, 0);
        }
    }
}

fn main() {
    // check for commandline args
    let args: Vec<String> = env::args().collect();

    let (_width, height) = tui_gen::tsize();
    let mut termstat = TermStat::default();
    termstat.height = height;

    tui_gen::cls();
    println!("fntoc: v{}\n", env!("CARGO_PKG_VERSION"));

    if args.len() < 2 {
        // get list of files in cwd
        for entry in glob("*").expect("Failed to read glob pattern") {
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

    gh_repo_status::check_version()
        .expect("check_version error");
}

fn find(path: &Path, termstat: &mut TermStat) {
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
        if line.contains("fn ") {
            let mut l = line.clone();
            if l.ends_with("{") {
                l.pop();
            }
            println!("{:>5} : {}", l_num.to_string().red(), l.trim_end());
            termstat.line_check();
        }
        if line.contains("struct ") {
            let mut l = line.clone();
            if l.ends_with("{") {
                l.pop();
            }
            println!("{:>5} : {}", l_num.to_string().red(), l.trim_end().yellow());
            termstat.line_check();
        }
        if line.starts_with("use ") {
            let mut l = line.clone();
            if l.ends_with("{") {
                l.pop();
            }
            println!("{:>5} : {}", l_num.to_string().red(), l.trim_end().cyan());
            termstat.line_check();
        }
        if line.starts_with("mod ") {
            let mut l = line.clone();
            if l.ends_with("{") {
                l.pop();
            }
            println!("{:>5} : {}", l_num.to_string().red(), l.trim_end().magenta());
            termstat.line_check();
        }
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
