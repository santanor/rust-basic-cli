use structopt::StructOpt;
use std::path::PathBuf;
use std::fs::{read_to_string, File};
use std::io::{BufReader, BufRead, BufWriter, self, Write};
use anyhow::{Context, Result};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: PathBuf,
}

fn main() -> Result<()>{
    let stdout = io::stdout();
    let mut handle = BufWriter::new(stdout);


    let args:Cli = Cli::from_args();
    let file = File::open(&args.path).with_context(||  format!("Could not read file `{:?}`", &args.path)).unwrap();
    let mut reader = BufReader::new(file);

    for lr in reader.lines(){
        let line = lr.with_context(|| format!("Something weird happened"))?;
        if line.contains(&args.pattern) {
            writeln!(handle, "{:?}", line)?;
        }
    }

    Ok(())
}
