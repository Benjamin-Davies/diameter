use crate::notes::LetterNote;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Scale(pub LetterNote);

#[cfg(test)]
mod test {
    use crate::{
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
