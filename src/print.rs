use std::{
    io::{self, Write},
    path::Path,
    process::{Command, Stdio},
};

use crate::chordpro::charts::{Chart, Line};

impl Chart {
    pub fn print_to_pdf(&self, output: &Path) -> io::Result<()> {
        let mut child = Command::new("typst")
            .arg("compile")
            .arg("-")
            .arg(output)
            .stdin(Stdio::piped())
            .spawn()?;

        let mut stdin = child
            .stdin
            .take()
            .ok_or_else(|| io::Error::other("unable to open stdin of child process"))?;
        self.print_to_typst(&mut stdin)?;
        drop(stdin);

        let status = child.wait()?;
        if !status.success() {
            return Err(io::Error::other(format!(
                "typst process exited with status: {status}"
            )));
        }

        Ok(())
    }

    pub fn print_to_typst(&self, mut f: impl Write) -> io::Result<()> {
        writeln!(f, r#"#import "@preview/chordx:0.6.1": single-chord"#)?;

        writeln!(f, r#"#set text(font: "Arial")"#)?;
        if let Some(title) = &self.title() {
            writeln!(f, "= {title}")?;
        }
        if let Some(comment) = &self.comment() {
            writeln!(f, "{comment}")?;
        }

        writeln!(f, r#"#set text(font: "Courier New")"#)?;
        writeln!(
            f,
            r#"
                #let chord = single-chord.with(
                    weight: "semibold",
                )
            "#
        )?;

        for line in &self.lines {
            match line {
                Line::Directive(_) => {}
                Line::Content { chunks, inline: _ } => {
                    for chunk in chunks {
                        let lyrics = &chunk.lyrics;
                        if let Some(chord) = &chunk.chord {
                            let offset = if !lyrics.trim().is_empty() { "1" } else { "" };
                            write!(f, r#"#chord[#"{lyrics}"][#"{chord} "][{offset}]"#)?;
                        } else {
                            write!(f, "{lyrics}")?;
                        }
                    }
                    writeln!(f, r"\")?;
                }
            }
        }

        Ok(())
    }
}
