use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct MidiPitch(u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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
}

impl Accidental {
    pub const DOUBLE_FLAT: Accidental = Accidental(-2);
    pub const FLAT: Accidental = Accidental(-1);
    pub const NATURAL: Accidental = Accidental(0);
    pub const SHARP: Accidental = Accidental(1);
    pub const DOUBLE_SHARP: Accidental = Accidental(2);

    pub const fn as_int(self) -> i8 {
        self.0
    }
}

impl FromStr for LetterNote {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        anyhow::ensure!(!s.is_empty(), "Cannot parse empty string as LetterNote");
        let split_index = s.chars().next().unwrap().len_utf8();
        let (letter_str, accidental_str) = s.split_at(split_index);
        let letter = letter_str.parse()?;
        let accidental = accidental_str.parse()?;

        Ok(LetterNote(letter, accidental))
    }
}

impl FromStr for Letter {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        match s {
            "C" => Ok(Letter::C),
            "D" => Ok(Letter::D),
            "E" => Ok(Letter::E),
            "F" => Ok(Letter::F),
            "G" => Ok(Letter::G),
            "A" => Ok(Letter::A),
            "B" => Ok(Letter::B),
            _ => anyhow::bail!("Invalid letter: {s:?}"),
        }
    }
}

impl FromStr for Accidental {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let mut accidental = 0;
        for c in s.chars() {
            match c {
                '#' => accidental += 1,
                'b' => accidental -= 1,
                _ => anyhow::bail!("Invalid accidental character: {c:?}"),
            }
        }
        Ok(Accidental(accidental))
    }
}

#[cfg(test)]
mod test {
    use crate::notes::{Accidental, Letter, LetterNote, MidiPitch};

    use Letter::*;

    const DOUBLE_FLAT: Accidental = Accidental::DOUBLE_FLAT;
    const FLAT: Accidental = Accidental::FLAT;
    const NATURAL: Accidental = Accidental::NATURAL;
    const SHARP: Accidental = Accidental::SHARP;
    const DOUBLE_SHARP: Accidental = Accidental::DOUBLE_SHARP;

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

    #[test]
    fn test_parse_letter_note() {
        assert_eq!("C".parse::<LetterNote>().unwrap(), LetterNote(C, NATURAL));
        assert_eq!("D#".parse::<LetterNote>().unwrap(), LetterNote(D, SHARP));
        assert_eq!(
            "Ebb".parse::<LetterNote>().unwrap(),
            LetterNote(E, DOUBLE_FLAT)
        );
        assert_eq!(
            "F##".parse::<LetterNote>().unwrap(),
            LetterNote(F, DOUBLE_SHARP)
        );
        assert_eq!("Db".parse::<LetterNote>().unwrap(), LetterNote(D, FLAT));
    }
}
