use std::fmt;

use crate::theory::notes::{Accidental, Letter, LetterNote, MidiPitch, Note};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Scale(pub LetterNote);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScaleDegree(u8, Accidental);

impl ScaleDegree {
    pub fn new(degree: u8, accidental: Accidental) -> Self {
        assert!(
            1 <= degree && degree <= 7,
            "Scale degree must be between 1 and 7"
        );
        ScaleDegree(degree, accidental)
    }

    pub fn in_key(self, key: Scale) -> LetterNote {
        let letter = key.0.letter() + (self.0 - 1) as i8;
        LetterNote(letter, Accidental::NATURAL).add_accidentals_to_match(self.midi_in_key(key))
    }

    pub fn midi_in_key(self, key: Scale) -> MidiPitch {
        let delta = match self.0 {
            1 => 0,
            2 => 2,
            3 => 4,
            4 => 5,
            5 => 7,
            6 => 9,
            7 => 11,
            _ => unreachable!(),
        };
        key.0.as_midi() + delta + self.1.as_int()
    }

    pub fn add_accidentals_to_match(self, key: Scale, target: MidiPitch) -> Self {
        let mut delta = (target.as_int() - self.in_key(key).as_midi().as_int()).rem_euclid(12);
        if delta > 6 {
            delta -= 12;
        }
        Self(self.0, Accidental::new(delta))
    }
}

impl Note {
    pub fn as_scale_degree(self, key: Scale) -> ScaleDegree {
        match self {
            Note::Letter(n) => n.as_scale_degree(key),
            Note::Number(n) => n,
        }
    }
}

impl LetterNote {
    pub fn as_scale_degree(self, key: Scale) -> ScaleDegree {
        self.letter()
            .as_natural_scale_degree(key)
            .add_accidentals_to_match(key, self.as_midi())
    }
}

impl Letter {
    pub fn as_natural_scale_degree(self, key: Scale) -> ScaleDegree {
        let key_letter = key.0.letter();
        let degree = (self.as_int() as i8 - key_letter.as_int() as i8).rem_euclid(7) as u8 + 1;
        ScaleDegree(degree, Accidental::NATURAL)
    }
}

impl fmt::Display for Scale {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for ScaleDegree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.1, self.0)
    }
}

#[cfg(test)]
mod test {
    use crate::theory::{
        notes::{Accidental, Letter, LetterNote},
        scales::Scale,
    };

    use Letter::*;

    const DOUBLE_FLAT: Accidental = Accidental::DOUBLE_FLAT;
    const FLAT: Accidental = Accidental::FLAT;
    const NATURAL: Accidental = Accidental::NATURAL;
    const SHARP: Accidental = Accidental::SHARP;
    const DOUBLE_SHARP: Accidental = Accidental::DOUBLE_SHARP;

    #[test]
    fn test_parse_scale() {
        assert_eq!("C".parse::<Scale>().unwrap(), Scale(LetterNote(C, NATURAL)));
        assert_eq!("D#".parse::<Scale>().unwrap(), Scale(LetterNote(D, SHARP)));
        assert_eq!(
            "Ebb".parse::<Scale>().unwrap(),
            Scale(LetterNote(E, DOUBLE_FLAT))
        );
        assert_eq!(
            "F##".parse::<Scale>().unwrap(),
            Scale(LetterNote(F, DOUBLE_SHARP))
        );
        assert_eq!("Db".parse::<Scale>().unwrap(), Scale(LetterNote(D, FLAT)));
    }
}
