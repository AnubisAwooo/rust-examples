use clap::Parser;
use rgrep::args::Args;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    dbg!(&args);

    let word = args.word;
    let files = args.files.unwrap_or_default();

    for file in files {
        grep_file(&word, &file)?;
    }

    Ok(())
}

fn grep_file(word: &str, file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut lines = read_lines(file)?;
    let mut row = 1;
    while let Some(line) = lines.next() {
        let line = line?;
        if line.contains(word) {
            println!("{}: {}", row, line);
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
    let file = std::fs::File::open(filename)?;
    Ok(std::io::BufReader::new(file).lines())
}
