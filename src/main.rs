use octocrab::{self, params::issues::Sort};
use std::fs;
use std::io::Write;
use tokio;
use tokio::time::{Duration, sleep};

use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::{LinesWithEndings, as_24_bit_terminal_escaped};

use clap::Parser;

#[derive(Parser)]
struct Arguments {
    #[arg(long, default_value = "0")]
    page: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Arguments::parse();

    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    let octo = octocrab::instance().user_access_token(std::env::var("GH_TOKEN__").unwrap())?;
    let mut page = octo
        .issues("rust-lang", "rust-clippy")
        .list()
        .sort(Sort::Created)
        .per_page(100)
        .page(args.page)
        .send()
        .await?;

    for item in page.items {
        if fs::Path::exists(format!("issues_repros/id{}.rs", item.number)) {
            continue;
        }
        if item.assignee.is_none()
            && item.comments == 0
            && item
                .labels
                .iter()
                .map(|l| l.name.clone())
                .collect::<Vec<String>>()
                .contains(&String::from("C-bug"))
        {
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
                    "ISSUE #{} - {}",
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
            } else if inp.to_lowercase().trim() == "n" {
                println!("User said no, Continuing...");
                sleep(Duration::from_secs(1)).await;
                continue;
            }
        };
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
