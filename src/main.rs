use which::which;
use colored::Colorize;
use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Show only installed fetches
    #[clap(long = "installed-only", short = 'i', action)]
    installed_only: bool,
}

fn fetch_installed(fetch: &str) -> bool {
    which(fetch).is_ok()
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

    for fetch in &FETCHES {

        if fetch_installed(fetch) {
            println!("{:>20} {}", fetch.on_cyan().black(), "is installed".green());
            fetch_count += 1;
        }
        else if !args.installed_only {
            println!("{:>20} {}", fetch.on_cyan().black(), "is not installed".red());
        }
    }

    println!("\n{}", format!("Fetches installed: {}", fetch_count).cyan());
    if fetch_count == 0 {
        println!("{}", "OMG no fetches installed!!!".red().blink())
    }
}
