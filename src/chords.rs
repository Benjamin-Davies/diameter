use std::fmt;

use crate::notes::LetterNote;

#[derive(Clone, PartialEq, Eq)]
pub struct Chord(pub LetterNote, pub ChordQuality);

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ChordQuality(pub String);

impl LetterNote {
    pub fn major_chord(self) -> Chord {
        Chord(self, ChordQuality::default())
    }

    pub fn minor_chord(self) -> Chord {
        Chord(self, ChordQuality("m".to_string()))
    }
}

impl fmt::Debug for Chord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Chord({self})")
    }
}

impl fmt::Display for Chord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.0, self.1)
    }
}

impl fmt::Display for ChordQuality {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
