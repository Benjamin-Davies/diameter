use std::{fs, path::PathBuf};

use clap::Parser;
use diameter::{charts::Chart, scales::Scale};

#[derive(Parser)]
struct Cli {
    /// The ChordPro file to process
    input: PathBuf,
    /// The output file (defaults to stdout)
    #[arg(short, long)]
    output: Option<PathBuf>,
    /// Output chords inline with lyrics
    #[arg(short, long)]
    inline: bool,
    /// Transpose the song into a different key
    #[arg(short, long)]
    key: Option<Scale>,
    /// Convert letter chords to numbers
    #[arg(short, long)]
    numbers: bool,
}

fn main() {
    let cli = Cli::parse();

    let input = fs::read_to_string(&cli.input).expect("unable to read input file");
    let mut chart = input
        .parse::<Chart>()
        .expect("unable to parse ChordPro file");

    chart.set_inline(cli.inline);
    if let Some(new_key) = cli.key {
        chart.transpose_to(new_key);
    }
    if cli.numbers {
        chart.to_numbers();
    }

    if let Some(output) = cli.output {
        fs::write(output, chart.to_string()).expect("unable to write output file");
    } else {
        print!("{chart}");
    }
}
