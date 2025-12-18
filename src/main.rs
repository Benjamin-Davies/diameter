use std::{fs, path::PathBuf};

use clap::Parser;
use diameter::charts::Chart;

#[derive(Parser)]
struct Cli {
    /// The ChordPro file to process
    input: PathBuf,
    /// Output chords inline with lyrics
    #[arg(short, long)]
    inline: bool,
}

fn main() {
    let cli = Cli::parse();

    let input = fs::read_to_string(&cli.input).expect("unable to read input file");
    let mut chart = input
        .parse::<Chart>()
        .expect("unable to parse ChordPro file");

    chart.set_inline(cli.inline);

    print!("{chart}");
}
