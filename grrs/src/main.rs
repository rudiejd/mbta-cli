use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead};
use anyhow::{Context, Result};
use std::path::Path;

#[derive(Parser)]
struct Command {
    pattern: String,
    path: std::path::PathBuf
}

fn read_file<P>(filename: &P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn find_matches<B>(lines: std::io::Lines<B>, pattern: &str, mut writer: impl std::io::Write)
-> anyhow::Result<()>
where B: BufRead 
{
    for (ix, line) in lines.enumerate() {
        if let Ok(p) = line {
            if p.contains(pattern) {
                writeln!(writer, "{}: {}", ix + 1, p)?
            } 
        }
    }
    Ok(())
}  

fn main() -> Result<()> {
    let command = Command::parse();

    let lines = read_file(&command.path)
        .with_context(|| {
            format!("could not read file {}", command.path.display())
        })?;
    
    find_matches(lines, &command.pattern, std::io::stdout()).with_context(|| {
        format!("error writing to file {}", command.path.display())
    })?;
    Ok(())
}

#[test]
fn find_matches_contains_matches_finds_matches() {
    let mut result = Vec::new();
    find_matches(io::BufReader::new(
        "yo what's up\nbeep\nyo, yo, yo".as_bytes()).lines(), "yo", &mut result).expect("this should find matches");
    let expeced_result = "1: yo what's up\n3: yo, yo, yo\n".as_bytes().to_vec();
    assert_eq!(expeced_result, result);
}
