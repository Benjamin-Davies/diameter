use std::{fs, path::PathBuf};

use clap::Parser;
use diameter::{chordpro::charts::Chart, theory::scales::Scale};

#[derive(Parser)]
struct Cli {
    /// The ChordPro file to process
    input: PathBuf,
    /// The output file (defaults to stdout)
    #[arg(short, long)]
    output: Option<PathBuf>,
    /// Print the chart as a PDF file
    #[arg(short, long)]
    #[cfg(feature = "print")]
    pdf_output: Option<PathBuf>,
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

    let mut did_output = false;
    if let Some(output) = cli.output {
        fs::write(output, chart.to_string()).expect("unable to write output file");
        did_output = true;
    }
    #[cfg(feature = "print")]
    if let Some(pdf_output) = cli.pdf_output {
        chart
            .print_to_pdf(&pdf_output)
            .expect("unable to print to PDF");
        did_output = true;
    }

    if !did_output {
        print!("{chart}");
    }
}
