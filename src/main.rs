#![feature(let_chains)]
use octocrab::{self, params::{issues::Sort, State}};
use std::fs::{self, read_to_string};
use std::io::Write;
use std::path::Path;
use tokio;
use tokio::time::{Duration, sleep};
use std::process::Command;

use owo_colors;

use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::{LinesWithEndings, as_24_bit_terminal_escaped};

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
    ignore_comment_count: bool
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Arguments::parse();

    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

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
        if Path::new(&format!("issues_repros/id{}.rs", item.number)).exists() {
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
            let Some(after_repro) = item_body.split_once("### Reproducer") else {
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
                "//{}\n\n{}",
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
                just_repro
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
        "All the items from page {} have been analyzed, proceeding to test them",
        args.page
    );

    for id in to_review {
        println!("Testing {id}...");
    
        dbg!(&Path::new(&std::env::current_dir().unwrap()).join("issues_repros").join(format!("id{}.rs", id.to_string())).display().to_string());

        // let output = dbg!(Command::new("zsh").args([format!("id{}.rs", id.to_string()))).output().unwrap();

        let output = Command::new("clippy-driver")
            .arg("-Dclippy::all")
            .arg(&Path::new(&std::env::current_dir().unwrap()).join("issues_repros").join(format!("id{}.rs", id.to_string())).display().to_string())
            // .env("LD_LIBRARY_PATH", "/home/meow/.rustup/toolchains/nightly-2025-06-12-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib");
            .output().expect("Failed to start the cargo command");
        if let Some(code) = output.status.code() && code != 1 { // Other errors are 130 and 131
            println!("{id} Couldn't be reproduced, what happened? Checkout <https://github.com/rust-lang/rust-clippy/issues/{id}>");
        } else {
            println!("{id} reproduces!");
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
    print!("\033[0m");
}

fn only_test_repro(ps: &SyntaxSet, ts: &ThemeSet) {
    let paths = fs::read_dir("issues_repros").unwrap();
    for path in paths {
        let path = path.unwrap();
        println!("Testing {}", path.path().to_string_lossy());

        let output = Command::new("clippy-driver")
        .arg("-Dclippy::all")
        .arg(path.path())
        // .env("LD_LIBRARY_PATH", "/home/meow/.rustup/toolchains/nightly-2025-06-12-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib");
        .output().expect("Failed to start the cargo command");
    if let Some(code) = output.status.code() && code != 1 { // Other errors are 130 and 131
        println!("{} Couldn't be reproduced, what happened? Checkout <https://github.com/rust-lang/rust-clippy/issues/{}>", &path.file_name().into_string().unwrap()[2..][..5], &path.file_name().into_string().unwrap()[2..][..5]);
        print_with_highlight(&read_to_string(path.path()).unwrap(), ps, ts);
        println!("PRESS ENTER TO CONTINUE");
        let mut inp = String::with_capacity(64);
        std::io::stdin()
            .read_line(&mut inp)
            .expect("Something went wrong while reading input");
    } else {
        println!("{} reproduces!", &path.file_name().into_string().unwrap()[1..][..5]);
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
}

fn bisect() {
    let paths = fs::read_dir("issues_repros").unwrap();
    for path in paths {
        let path = path.unwrap();
        println!("BISECTING {}", path.path().to_string_lossy());

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
