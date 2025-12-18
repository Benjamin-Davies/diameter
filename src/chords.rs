use std::fmt;

use crate::notes::{LetterNote, Note};

#[derive(Clone, PartialEq, Eq)]
pub struct Chord {
    pub root: Note,
    pub quality: ChordQuality,
    pub bass: Option<Note>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ChordQuality(pub String);

impl Chord {
    pub fn major(root: impl Into<Note>) -> Chord {
        Chord {
            root: root.into(),
            quality: ChordQuality::default(),
            bass: None,
        }
    }

    pub fn minor(root: impl Into<Note>) -> Chord {
        Chord {
            root: root.into(),
            quality: ChordQuality("m".to_string()),
            bass: None,
        }
    }

    pub fn over(self, bass: impl Into<Note>) -> Chord {
        Chord {
            bass: Some(bass.into()),
            ..self
        }
    }
}

impl LetterNote {
    pub fn major_chord(self) -> Chord {
        Chord::major(self)
    }

    pub fn minor_chord(self) -> Chord {
        Chord::minor(self)
    }
}

impl fmt::Debug for Chord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Chord({self})")
    }
}

impl fmt::Display for Chord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.root, self.quality)?;
        if let Some(bass) = &self.bass {
            write!(f, "/{bass}")?;
        }
        Ok(())
    }
}

impl fmt::Display for ChordQuality {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
