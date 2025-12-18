use std::fmt::{self, Write};

use crate::{chords::Chord, directives::Directive};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Chart {
    pub lines: Vec<Line>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Line {
    Directive(Directive),
    Content { chunks: Vec<Chunk>, inline: bool },
}

impl Line {
    pub fn is_empty(&self) -> bool {
        match self {
            Line::Directive { .. } => false,
            Line::Content { chunks, .. } => chunks.is_empty(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Chunk {
    pub chord: Option<Chord>,
    pub lyrics: String,
}

impl Chart {
    pub fn set_inline(&mut self, inline: bool) {
        for line in &mut self.lines {
            if let Line::Content { inline: i, .. } = line {
                *i = inline;
            }
        }
    }
}

impl fmt::Display for Chart {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in &self.lines {
            writeln!(f, "{line}")?;
        }
        Ok(())
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Line::Directive(directive) => write!(f, "{directive}"),
            Line::Content { chunks, inline } => {
                if *inline {
                    for chunk in chunks {
                        write!(f, "{chunk}")?;
                    }
                } else {
                    let mut index = 0;
                    let mut chord_line = String::new();
                    let mut lyric_line = String::new();
                    for chunk in chunks {
                        if chunk.chord.is_some() {
                            while chord_line.len() < index {
                                chord_line.push(' ');
                            }
                        }
                        if !chunk.lyrics.is_empty() {
                            while lyric_line.len() < index {
                                lyric_line.push(' ');
                            }
                        }

                        if let Some(chord) = &chunk.chord {
                            write!(&mut chord_line, "{chord}")?;
                            index = chord_line.len() + 1;
                        }
                        lyric_line.push_str(&chunk.lyrics);
                        index = index.max(lyric_line.len());
                    }

                    if !chord_line.is_empty() {
                        writeln!(f, "{chord_line}")?;
                    }
                    write!(f, "{lyric_line}")?;
                }
                Ok(())
            }
        }
    }
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(chord) = &self.chord {
            write!(f, "[{chord}]")?;
        }
        write!(f, "{}", self.lyrics)
    }
}
