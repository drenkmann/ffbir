use which::which;
use colored::Colorize;

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

    let mut fetch_count: i8 = 0;

    println!("{}\n", "### FetchFetch but in Rust ###".green().bold());

    for fetch in &FETCHES {
        print!("{:>20} ", fetch.on_cyan().black());

        if fetch_installed(fetch) {
            println!("{}", "is installed".green());
            fetch_count += 1;
        }
        else {
            println!("{}", "is not installed".red());
        }
    }

    println!("\n{}", format!("Fetches installed: {}", fetch_count).cyan());
    if fetch_count == 0 {
        println!("{}", "OMG no fetches installed!!!".red().blink())
    }
}
