use clap::Parser;
use colored::*;
use rgrep::{args::Args, read::parse_read_lines, search::search};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // dbg!(&args);

    let word = args.word;
    let files = args.files.unwrap_or_default();

    let sources = parse_read_lines(files);

    let handlers = sources
        .into_iter()
        .map(|source| {
            let word = word.clone();
            std::thread::spawn(|| search(source, word))
        })
        .collect::<Vec<_>>();

    let length = handlers.len();
    for handler in handlers {
        if let Ok((source, contents)) = handler.join() {
            if args.nothing {
                show(length, source, contents);
            } else if !contents.is_empty() {
                show(length, source, contents);
            }
        }
    }

    Ok(())
}

fn show(len: usize, source: String, contents: Vec<String>) {
    if len > 1 {
        println!(
            "file: {} found: {}",
            source.bold().blue(),
            contents.len().to_string().blue()
        );
        contents.iter().for_each(|s| println!("  {}", s));
    } else {
        println!("found: {}", contents.len().to_string().blue());
        contents.iter().for_each(|s| println!("{}", s));
    }
}
