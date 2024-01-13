use clap::Parser;
use colored::*;
use regex::Regex;
use rgrep::args::Args;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    dbg!(&args);

    let word = args.word;
    let files = args.files.unwrap_or_default();

    let re = Regex::new(&word).unwrap();

    for file in files {
        grep_file(&re, &file)?;
    }

    Ok(())
}

fn grep_file(re: &Regex, file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut lines = read_lines(file)?;
    let mut row = 1;
    while let Some(line) = lines.next() {
        let line = line?;
        if let Some(cap) = re.captures(&line) {
            let mut iter = cap.iter();
            while let Some(Some(m)) = iter.next() {
                let start = m.start();
                let end = m.end();
                println!(
                    "    {row}:{start} {}{}{}",
                    line.split_at(start).0,
                    line.split_at(start).1.split_at(end - start).0.red(),
                    line.split_at(end).1
                );
            }
        }
        row += 1;
    }
    Ok(())
}

fn read_lines<P>(filename: P) -> std::io::Result<std::io::Lines<std::io::BufReader<std::fs::File>>>
where
    P: AsRef<std::path::Path>,
{
    use std::io::BufRead;
    println!("{}", filename.as_ref().to_str().unwrap());
    let file = std::fs::File::open(filename)?;
    Ok(std::io::BufReader::new(file).lines())
}
