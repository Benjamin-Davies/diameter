use crate::{chords::Chord, directives::Directive};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Chart(pub Vec<Line>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Line {
    Directive(Directive),
    Content(Vec<Chunk>),
}

impl Line {
    pub fn is_empty(&self) -> bool {
        match self {
            Line::Directive(_) => false,
            Line::Content(chunks) => chunks.is_empty(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Chunk(pub Option<Chord>, pub String);
