use std::fmt::{self, Write};

use crate::{
    chordpro::directives::Directive,
    theory::{chords::Chord, notes::Note, scales::Scale},
};

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
    pub fn title(&self) -> Option<&str> {
        for line in &self.lines {
            if let Line::Directive(Directive::Title(title)) = line {
                return Some(title);
            }
        }
        None
    }

    pub fn comment(&self) -> Option<&str> {
        for line in &self.lines {
            if let Line::Directive(Directive::Comment(comment)) = line {
                return Some(comment);
            }
        }
        None
    }

    pub fn key(&self) -> Option<Scale> {
        for line in &self.lines {
            if let &Line::Directive(Directive::Key(key)) = line {
                return Some(key);
            }
        }
        None
    }

    pub fn set_key(&mut self, key: Scale) {
        for line in &mut self.lines {
            if let Line::Directive(Directive::Key(k)) = line {
                *k = key;
                return;
            }
        }

        let after_directives = self
            .lines
            .iter()
            .position(|line| !matches!(line, Line::Directive(_)))
            .unwrap_or(self.lines.len());
        self.lines
            .insert(after_directives, Line::Directive(Directive::Key(key)));
    }

    pub fn set_inline(&mut self, inline: bool) {
        for line in &mut self.lines {
            if let Line::Content { inline: i, .. } = line {
                *i = inline;
            }
        }
    }

    pub fn to_numbers(&mut self) {
        let key = self
            .key()
            .expect("cannot convert to numbered notation without a key");
        self.transform_all_notes(|note| note.as_scale_degree(key).into());
    }

    pub fn transpose_to(&mut self, new_key: Scale) {
        let old_key = self.key().expect("cannot transpose without a key");
        self.transform_all_notes(|note| note.as_scale_degree(old_key).in_key(new_key).into());
        self.set_key(new_key);
    }

    fn transform_all_notes<F>(&mut self, mut f: F)
    where
        F: FnMut(&Note) -> Note,
    {
        self.transform_all_chords(|chord| Chord {
            root: f(&chord.root),
            quality: chord.quality.clone(),
            bass: chord.bass.as_ref().map(|b| f(b)),
        });
    }

    fn transform_all_chords<F>(&mut self, mut f: F)
    where
        F: FnMut(&Chord) -> Chord,
    {
        for line in &mut self.lines {
            if let Line::Content { chunks, .. } = line {
                for chunk in chunks {
                    if let Some(chord) = &mut chunk.chord {
                        *chord = f(chord);
                    }
                }
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

#[cfg(test)]
mod tests {
    use crate::chordpro::charts::Chart;

    const O_HOLY_NIGHT: &str = include_str!("../../examples/O-Holy-Night-.chordpro");
    const O_HOLY_NIGHT_BFLAT: &str = include_str!("../../examples/O-Holy-Night-Bb.chordpro");

    #[test]
    fn test_transpose() {
        let mut chart = O_HOLY_NIGHT.parse::<Chart>().unwrap();
        chart.transpose_to("Bb".parse().unwrap());
        assert_eq!(format!("{chart}"), O_HOLY_NIGHT_BFLAT);
    }
}
