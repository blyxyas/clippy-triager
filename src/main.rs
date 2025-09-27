#![feature(f128)]
#![feature(let_chains)]
use octocrab::{self, Octocrab, params::{Direction, State, issues::Sort, pulls}};
use std::fs::{self, read_to_string};
use std::io::Write;
use std::path::Path;
use tokio;
use tokio::time::{Duration, sleep};
use std::process::Command;

use chrono::NaiveDate;
use owo_colors::{Style as OwoStyle, OwoColorize};

use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::{LinesWithEndings, as_24_bit_terminal_escaped};

use regex::Regex;

use clap::Parser;

#[derive(Parser)]
struct Arguments {
    #[arg(long, default_value = "0")]
    page: u32,
    /// Path to your clippy checkout
    #[arg(long, default_value = "../rust-clippy")]
    clippy: String,
    #[arg(long, default_value = "false")]
    bisect: bool,
    #[arg(long, default_value = "false")]
    repro: bool,
    #[arg(long, default_value = "false")]
    ignore_comment_count: bool,
    /// Turn the tool into profiling mode (incompatible with every other thing, needs Callgrind)
    #[arg(long, default_value = "false")]
    profile: bool,
    /// ID of the PR to profile (needs Callgrind)
    #[arg(long, default_value = "0")]
    profile_pr: usize,
    #[arg(long, default_value = "")]
    ld_lib_path: String,
    #[arg(long, default_value = "-Wclippy::all")]
    rustflags: String,
    #[arg(long, default_value = "false")]
    pr_history: bool,
    #[arg(long, default_value = "false")]
    pr_history_read: bool
}

const COMPLETE: owo_colors::Style = OwoStyle::new()
        .green()
        .bold();

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Arguments::parse();

    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    if args.profile {
        if args.profile_pr == 0 {
            panic!("--profile-pr needs to be provided");
        }
        if args.ld_lib_path == "" {
            panic!("--lib-lib-path needs to be provided");
        }

        profile(args.profile_pr, args.ld_lib_path, args.rustflags)?;
        return Ok(());
    }

    if args.bisect {
        bisect();
        return Ok(())
    }

    if args.repro {
        only_test_repro(&ps, &ts);
        return Ok(())
    }


    sleep(Duration::from_secs(5)).await; // Sleep 5 seconds to make sure that Github doesn't rate limit us

    let octo = octocrab::instance().user_access_token(std::env::var("GH_TOKEN__").unwrap())?;

    if args.pr_history {
        for i in 0..=50 {
            if let Ok(()) = pr_history(Box::new(octo.clone()), Box::new(i), args.pr_history_read).await {
                return Ok(())
            }
        };
        return Ok(())
    }

    let page = octo
        .issues("rust-lang", "rust-clippy")
        .list()
        .labels(&[String::from("C-bug")])
        .assignee("none")
        .state(State::Open)
        .sort(Sort::Created)
        .per_page(100)
        .page(args.page)
        .send()
        .await?;
    
    let mut to_review: Vec<u64> = Vec::new();

    for item in page.items {
        if Path::new(&format!("issues_repros/id{}.rs", item.number)).exists() || Path::new(&format!("issues_repros/triaged/id{}.rs", item.number)).exists() {
            continue;
        }
        // Just some redundancy here to avoid false positives in the false positive finder lol
        if item.assignee.is_none()
            && item.pull_request.is_none()
            && item
                .labels
                .iter()
                .map(|l| l.name.clone())
                .collect::<Vec<String>>()
                .contains(&String::from("C-bug"))
        {
            if !args.ignore_comment_count && item.comments != 0 {
                continue;
            }

            dbg!(&item.number);
            let item_body = item.body.clone().unwrap();
            let Some(after_repro) = item_body.split_once("# Reproducer") else {
                println!("Without repro");
                continue;
            };
            let mut just_repro = after_repro.1.splitn(3, "```").collect::<Vec<&str>>()[1];
            if !just_repro.starts_with('\n') {
                let split_once = just_repro.split_once('\n').unwrap();
                if split_once.0.len() < 10 {
                    just_repro = split_once.1 // Very heuristic-y
                }
            }

            let concatenated = format!(
                "//{}\n\n{}\n{}",
                &format!(
                    "ISSUE #{} <https://github.com/rust-lang/rust-clippy/issues/{}> - {}",
                    item.number,
                    item.number,
                    item.labels
                        .iter()
                        .map(|label| label.name.clone())
                        .collect::<Vec<String>>()
                        .join(", ")
                ),
                just_repro,
                if just_repro.contains("main") { "" } else { "fn main() {}" }
            );
            print_with_highlight(&concatenated, &ps, &ts);

            println!("Agree with this? [Y/n]: ");
            let mut inp = String::with_capacity(64);
            std::io::stdin()
                .read_line(&mut inp)
                .expect("Something went wrong while reading input");

            if inp.to_lowercase().trim() == "y" {
                let mut file =
                    fs::File::create(format!("issues_repros/id{}.rs", item.number)).unwrap();
                file.write(concatenated.as_bytes()).unwrap();
                to_review.push(item.number);
            } else if inp.to_lowercase().trim() == "n" {
                println!("User said no, Continuing...");
                sleep(Duration::from_secs(1)).await;
                continue;
            }
        };
    }

    // REMOVE THIS LATER

    println!(
        "{} All the items from page {} have been analyzed, proceeding to test them",
        "Complete".style(COMPLETE),
        args.page
    );

    for id in to_review {
        println!("{} {id}...", "Testing".style(COMPLETE));
    
        dbg!(&Path::new(&std::env::current_dir().unwrap()).join("issues_repros").join(format!("id{}.rs", id.to_string())).display().to_string());

        // let output = dbg!(Command::new("zsh").args([format!("id{}.rs", id.to_string()))).output().unwrap();

        let output = Command::new("clippy-driver")
            .arg("-Dclippy::all")
            .arg("--emit=metadata")
            .arg(&format!("-otarget/id{}", id.to_string()))
            .arg("-Cembed-bitcode=no")
            .arg(&Path::new(&std::env::current_dir().unwrap()).join("issues_repros").join(format!("id{}.rs", id.to_string())).display().to_string())
            // .env("LD_LIBRARY_PATH", "/home/meow/.rustup/toolchains/nightly-2025-06-12-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib");
            .output().expect("Failed to start the cargo command");
        if let Some(code) = output.status.code() && code != 1 { // Other errors are 130 and 131
            println!("{}", format!("{id} Couldn't be reproduced, what happened? Checkout <https://github.com/rust-lang/rust-clippy/issues/{id}>").red().bold());
        } else {
            println!("{} {id} reproduces!", "Checking".style(COMPLETE));
   let s = match std::str::from_utf8(&output.stdout) {
                    Ok(v) => v,
                    Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                };
            // dbg!(s);

            let s = match std::str::from_utf8(&output.stderr) {
                    Ok(v) => v,
                    Err(e) => panic!("Invalid UTF-8 s"),
            };
            // dbg!(&s);
        }
    }

    Ok(())
}

fn print_with_highlight(s: &str, ps: &SyntaxSet, ts: &ThemeSet) {
    let syntax = ps.find_syntax_by_extension("rs").unwrap();
    let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);
    for line in LinesWithEndings::from(s) {
        let ranges: Vec<(Style, &str)> = h.highlight_line(line, &ps).unwrap();
        let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
        print!("{}", escaped);
    }
    print!("{}", "".on_default_color());
}

fn only_test_repro(ps: &SyntaxSet, ts: &ThemeSet) {
    let paths = fs::read_dir("issues_repros").unwrap();
    for path in paths {
        let path = path.unwrap();
        println!("{} {}", "Testing".style(COMPLETE), path.path().to_string_lossy());

        let output = Command::new("clippy-driver")
        .arg("-Awarnings")
        .arg("-Dclippy::all")
        .arg("--emit=metadata")
        .arg(&format!("-otarget/{}", path.file_name().to_string_lossy()))
        .arg("-Cembed-bitcode=no")
        .arg("--edition=2024")
        .arg(path.path())
        // .env("LD_LIBRARY_PATH", "/home/meow/.rustup/toolchains/nightly-2025-06-12-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib");
        .output().expect("Failed to start the cargo command");
    if let Some(code) = output.status.code() && code != 1 { // Other errors are 130 and 131
        println!("{} Couldn't be reproduced, what happened? Checkout <https://github.com/rust-lang/rust-clippy/issues/{}>", &path.file_name().into_string().unwrap()[2..][..5], &path.file_name().into_string().unwrap()[2..][..5]);
        
        let file_to_string = read_to_string(path.path()).unwrap();
        print_with_highlight(&file_to_string, ps, ts);
        println!("Move this to triaged? [y/n]");
        let mut inp = String::with_capacity(64);
        std::io::stdin()
            .read_line(&mut inp)
            .expect("Something went wrong while reading input");

        if inp.to_lowercase().trim() == "y" {
            fs::rename(path.path(), format!("issues_repros/triaged/{}", path.file_name().to_string_lossy())).unwrap();
        } else {
            println!("Counting as not triaged");
        }
    } else {
        println!("{} {} reproduces!", "Checking".style(COMPLETE), &path.file_name().into_string().unwrap()[2..][..5]);
let s = match std::str::from_utf8(&output.stdout) {
                Ok(v) => v,
                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
            };
            
            let s = match std::str::from_utf8(&output.stderr) {
                Ok(v) => v,
                Err(e) => panic!("Invalid UTF-8 s"),
            };
        if !s.contains("clippy") {
            println!("Something's wrong with {}? An error happens but not with Clippy", &path.file_name().into_string().unwrap()[2..][..5]);
        }
        // dbg!(&s);
    }
    }
}

fn bisect() {
    let paths = fs::read_dir("issues_repros").unwrap();
    for path in paths {
        let path = path.unwrap();
        println!("{} {}", "Bisecting".style(COMPLETE), path.path().to_string_lossy());

        let output = Command::new("cargo")
        .arg("bisect-rustc")
        .arg("-c")
        .arg("clippy")
        .arg("--")
        .arg("clippy-driver")
        .arg(path.path().display().to_string())
        
        .spawn().unwrap().wait_with_output().unwrap();

        if output.status.success() {
            dbg!("FOUND THE REGRESSION");
            match std::str::from_utf8(&output.stdout) {
                Ok(v) => println!("{}", v),
                Err(e) => panic!("Invalid UTF-8 s"),
            };
            match std::str::from_utf8(&output.stderr) {
                Ok(v) => println!("{}", v),
                Err(e) => panic!("Invalid UTF-8 s"),
            };
        }
    }
}

fn profile(pr: usize, lib_path: String, rustflags: String) -> Result<(), Box<dyn std::error::Error>>{
    let output = Command::new("git")
        .args(&["rev-parse", "--abbrev-ref", "HEAD"])
        .current_dir("/home/meow/git/rust-clippy")
        .output()?;

    let s = match std::str::from_utf8(&output.stdout) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    if s.trim() != "master" {
        Command::new("git")
        .args(&["switch", "master"])
        .current_dir("/home/meow/git/rust-clippy")
        .output()?;
    }

    Command::new("cargo")
        .args(&["build", "--release"])
        .current_dir("/home/meow/git/rust-clippy")
        .output()?;

    let output = Command::new("valgrind")
        .args(&["--tool=callgrind", "--dump-instr=yes", "--trace-children=yes", "../../../release/cargo-clippy"])
        .env("CARGO_TARGET_DIR", &format!("/tmp/mc{}master", pr))
        .env("RUSTFLAGS", &rustflags)
        .env("LD_LIBRARY_PATH", &lib_path)
        .current_dir(std::env::var("TO_PROFILE_PATH").unwrap())
        .output()?;

    let s = match std::str::from_utf8(&output.stderr) {
        Ok(v) => dbg!(v),
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    let re = Regex::new(r"Collected : (\d)*").unwrap();
    let mut master_ir_collected: i128 = 0;
    for cap in re.captures_iter(s) {
        dbg!(cap[0].split("Collected : ").collect::<Vec<&str>>());
        master_ir_collected += cap[0].split("Collected : ").collect::<Vec<&str>>()[1].parse::<i128>().unwrap();
    }

    dbg!(master_ir_collected);

    Command::new("gh")
        .args(&["pr", "checkout", &pr.to_string()])
        .current_dir("/home/meow/git/rust-clippy")
        .output()?;

    Command::new("cargo")
        .args(&["build", "--release"])
        .current_dir("/home/meow/git/rust-clippy")
        .output()?;

        let output = Command::new("valgrind")
        .args(&["--tool=callgrind", "--dump-instr=yes", "--trace-children=yes", "../../../release/cargo-clippy"])
        .env("CARGO_TARGET_DIR", &format!("/tmp/mc{}branch", pr))
        .env("RUSTFLAGS", rustflags)
        .env("LD_LIBRARY_PATH", lib_path)
        .current_dir(std::env::var("TO_PROFILE_PATH").unwrap())
        .output()?;

    let s = match std::str::from_utf8(&output.stderr) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    let re = Regex::new(r"Collected : (\d)*").unwrap();
    let mut branch_ir_collected: i128 = 0;
    for cap in re.captures_iter(s) {
        dbg!(cap[0].split("Collected : ").collect::<Vec<&str>>());
        branch_ir_collected += cap[0].split("Collected : ").collect::<Vec<&str>>()[1].parse::<i128>().unwrap();
    }
    
    dbg!(branch_ir_collected);

    std::fs::remove_dir_all(format!("/tmp/mc{}branch", pr)).unwrap();
    std::fs::remove_dir_all(format!("/tmp/mc{}master", pr)).unwrap();

    let result = ((master_ir_collected as f64 - branch_ir_collected as f64) / master_ir_collected as f64) * 100.0f64;
    if result.abs() >= 0.19f64 {
        println!("{}% {}", ((master_ir_collected as f64 - branch_ir_collected as f64) / master_ir_collected as f64) * 100.0f64, if master_ir_collected > branch_ir_collected {"THIS IS A PERFORMANCE IMPROVEMENT"} else {"PERF. REGRESSION"});
    } else {
        println!("Not noticeable ({})", result);
    }

    Ok(())
}

async fn pr_history(octo: Box<Octocrab>, page: Box<u32>, read: bool) -> Result<(), ()> {
    let pulls = octo
        .pulls("rust-lang", "rust-clippy")
        .list()
        .state(State::All)
        .sort(pulls::Sort::Created)
        .page(*page)
        .direction(Direction::Descending)
        .per_page(100)
        .send()
        .await.unwrap();

    for pull in pulls {
        // if pull.created_at.unwrap().date_naive() >= NaiveDate::from_ymd_opt(2025, 06, 26).unwrap()
        // && pull.created_at.unwrap().date_naive() <= NaiveDate::from_ymd_opt(2025, 09, 18).unwrap(){
            // println!("{} - {}", pull.url.split("/").last().unwrap(), pull.created_at.unwrap().date_naive());
        // }
        println!("{} - {}", pull.url.split("/").last().unwrap(), pull.user.unwrap().login);
        if pull.url.split("/").last().unwrap() == "5671" {
            
            return Ok(())
        }
    }

    println!("Sleeping...");
    sleep(Duration::from_secs(10)).await;
    Err(())
}
