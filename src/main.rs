use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::path::PathBuf;

use clap::Parser;
use colored::Colorize;
use which::which;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Show only installed fetches
    #[clap(long = "installed-only", short = 'i', action)]
    installed_only: bool,

    #[clap(long = "fetches", short = 'f')]
    fetches_file: Option<PathBuf>,
}

fn fetch_installed(fetch: &str) -> bool {
    which(fetch).is_ok()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    const FETCHES: [&str; 21] = [
        "fetchfetch",
        "shutthefetchup",
        "nofetch",
        "neofetch",
        "pfetch",
        "ramfetch",
        "ufetch",
        "nerdfetch",
        "archfetch",
        "cfetch",
        "onefetch",
        "hyfetch",
        "uwufetch",
        "picofetch",
        "macchina",
        "minifetch",
        "paleofetch",
        "cpufetch",
        "gpufetch",
        "fastfetch",
        "wfetch",
    ];

    let args = Cli::parse();
    let mut fetch_count: i8 = 0;

    println!("{}\n", "### FetchFetch but in Rust ###".green().bold());

    match args.fetches_file {
        None => {
            for fetch in &FETCHES {
                if fetch_installed(fetch) {
                    println!("{:>20} {}", fetch.on_cyan().black(), "is installed".green());
                    fetch_count += 1;
                } else if !args.installed_only {
                    println!(
                        "{:>20} {}",
                        fetch.on_cyan().black(),
                        "is not installed".red()
                    );
                }
            }
        }
        Some(path) => {
            if !path.exists() {
                println!("Fetches file not found!");
                std::process::exit(1);
            }
            if let Ok(lines) = read_lines(path.as_path()) {
                for line in lines {
                    if let Ok(fetch) = line {
                        if fetch_installed(&fetch) {
                            println!("{:>20} {}", fetch.on_cyan().black(), "is installed".green());
                            fetch_count += 1;
                        } else if !args.installed_only {
                            println!(
                                "{:>20} {}",
                                fetch.on_cyan().black(),
                                "is not installed".red()
                            );
                        }
                    }
                }
            }
        }
    }

    println!("\n{}", format!("Fetches installed: {}", fetch_count).cyan());
    if fetch_count == 0 {
        println!("{}", "OMG no fetches installed!!!".red().blink())
    }
}
