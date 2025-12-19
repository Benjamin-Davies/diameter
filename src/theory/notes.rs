use std::{fmt, ops::Add};

use crate::theory::scales::ScaleDegree;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct MidiPitch(u8);

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Note {
    Letter(LetterNote),
    Number(ScaleDegree),
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct LetterNote(pub Letter, pub Accidental);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Letter {
    C,
    D,
    E,
    F,
    G,
    A,
    B,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Accidental(i8);

impl MidiPitch {
    pub const fn as_int(self) -> i8 {
        self.0 as i8
    }

    pub const fn as_letter(self) -> LetterNote {
        let letter = match self.0 % 12 {
            0 => Letter::C,
            1 | 2 => Letter::D,
            3 | 4 => Letter::E,
            5 => Letter::F,
            6 | 7 => Letter::G,
            8 | 9 => Letter::A,
            10 | 11 => Letter::B,
            _ => unreachable!(),
        };
        LetterNote(letter, Accidental::NATURAL).add_accidentals_to_match(self)
    }
}

impl LetterNote {
    pub const fn letter(self) -> Letter {
        self.0
    }

    pub const fn accidental(self) -> Accidental {
        self.1
    }

    pub const fn as_midi(self) -> MidiPitch {
        let base_pitch = self.letter().as_midi().as_int();
        let pitch = base_pitch + self.accidental().as_int();
        MidiPitch(pitch as u8)
    }

    pub const fn add_accidentals_to_match(self, target: MidiPitch) -> LetterNote {
        let base_pitch = self.letter().as_midi().as_int();
        let target_pitch = target.as_int();
        let mut accidental = (target_pitch - base_pitch).rem_euclid(12);
        if accidental > 6 {
            accidental -= 12;
        }
        LetterNote(self.letter(), Accidental(accidental))
    }
}

impl Letter {
    pub const fn from_int(value: u8) -> Letter {
        match value % 7 {
            0 => Letter::C,
            1 => Letter::D,
            2 => Letter::E,
            3 => Letter::F,
            4 => Letter::G,
            5 => Letter::A,
            6 => Letter::B,
            _ => unreachable!(),
        }
    }

    pub const fn as_int(self) -> u8 {
        match self {
            Letter::C => 0,
            Letter::D => 1,
            Letter::E => 2,
            Letter::F => 3,
            Letter::G => 4,
            Letter::A => 5,
            Letter::B => 6,
        }
    }

    pub const fn as_midi(self) -> MidiPitch {
        let pitch = match self {
            Letter::C => 0,
            Letter::D => 2,
            Letter::E => 4,
            Letter::F => 5,
            Letter::G => 7,
            Letter::A => 9,
            Letter::B => 11,
        };
        MidiPitch(pitch + 60)
    }

    pub const fn double_flat(self) -> LetterNote {
        LetterNote(self, Accidental::DOUBLE_FLAT)
    }

    pub const fn flat(self) -> LetterNote {
        LetterNote(self, Accidental::FLAT)
    }

    pub const fn natural(self) -> LetterNote {
        LetterNote(self, Accidental::NATURAL)
    }

    pub const fn sharp(self) -> LetterNote {
        LetterNote(self, Accidental::SHARP)
    }

    pub const fn double_sharp(self) -> LetterNote {
        LetterNote(self, Accidental::DOUBLE_SHARP)
    }
}

impl Accidental {
    pub const DOUBLE_FLAT: Accidental = Accidental(-2);
    pub const FLAT: Accidental = Accidental(-1);
    pub const NATURAL: Accidental = Accidental(0);
    pub const SHARP: Accidental = Accidental(1);
    pub const DOUBLE_SHARP: Accidental = Accidental(2);

    pub fn new(delta: i8) -> Self {
        assert!(
            -2 <= delta && delta <= 2,
            "{delta} is too large to be an accidental"
        );
        Self(delta)
    }

    pub const fn as_int(self) -> i8 {
        self.0
    }
}

impl From<LetterNote> for Note {
    fn from(note: LetterNote) -> Self {
        Note::Letter(note)
    }
}

impl From<ScaleDegree> for Note {
    fn from(note: ScaleDegree) -> Self {
        Note::Number(note)
    }
}

impl From<u8> for Note {
    fn from(degree: u8) -> Self {
        Note::Number(ScaleDegree::new(degree, Accidental::NATURAL))
    }
}

impl From<(u8, Accidental)> for Note {
    fn from((degree, accidental): (u8, Accidental)) -> Self {
        Note::Number(ScaleDegree::new(degree, accidental))
    }
}

impl Add<i8> for MidiPitch {
    type Output = MidiPitch;

    fn add(self, rhs: i8) -> Self::Output {
        MidiPitch((self.as_int() + rhs) as u8)
    }
}

impl Add<Accidental> for MidiPitch {
    type Output = MidiPitch;

    fn add(self, rhs: Accidental) -> Self::Output {
        MidiPitch((self.as_int() + rhs.as_int()) as u8)
    }
}

impl Add<i8> for Letter {
    type Output = Letter;

    fn add(self, rhs: i8) -> Self::Output {
        Letter::from_int((self.as_int() as i8 + rhs).rem_euclid(7) as u8)
    }
}

impl fmt::Debug for Note {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Note::Letter(n) => write!(f, "{n:?}"),
            Note::Number(n) => write!(f, "{n:?}"),
        }
    }
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Note::Letter(n) => write!(f, "{n}"),
            Note::Number(n) => write!(f, "{n}"),
        }
    }
}

impl fmt::Debug for LetterNote {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LetterNote({self})")
    }
}

impl fmt::Display for LetterNote {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.0, self.1)
    }
}

impl fmt::Display for Letter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl fmt::Display for Accidental {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.0 < 0 {
            for _ in 0..-self.0 {
                write!(f, "b")?;
            }
        } else if self.0 > 0 {
            for _ in 0..self.0 {
                write!(f, "#")?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::theory::notes::{Accidental, Letter, LetterNote, MidiPitch};

    use Letter::*;

    const FLAT: Accidental = Accidental::FLAT;
    const NATURAL: Accidental = Accidental::NATURAL;

    #[test]
    fn test_midi_pitch_as_letter() {
        assert_eq!(MidiPitch(60).as_letter(), LetterNote(C, NATURAL));
        assert_eq!(MidiPitch(76).as_letter(), LetterNote(E, NATURAL));
        assert_eq!(MidiPitch(82).as_letter(), LetterNote(B, FLAT));
    }

    #[test]
    fn test_letter_note_as_midi() {
        assert_eq!(LetterNote(C, NATURAL).as_midi(), MidiPitch(60));
        assert_eq!(LetterNote(E, NATURAL).as_midi(), MidiPitch(64));
        assert_eq!(LetterNote(B, FLAT).as_midi(), MidiPitch(70));
    }
}
