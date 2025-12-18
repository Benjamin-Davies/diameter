use std::str::FromStr;

use nom::{
    AsChar, IResult, Parser,
    branch::alt,
    bytes::complete::{tag, take_until, take_while1},
    character::{
        complete::one_of,
        streaming::{alphanumeric0, line_ending},
    },
    combinator::{complete, eof, opt, success},
    multi::{many_till, many0},
};

use crate::{
    charts::{Chart, Chunk, Line},
    chords::{Chord, ChordQuality},
    directives::Directive,
    notes::{Accidental, Letter, LetterNote},
    scales::Scale,
};

type Error<'input> = nom::error::Error<&'input str>;
type OwnedError = nom::Err<nom::error::Error<String>>;

pub fn chart(input: &str) -> IResult<&str, Chart> {
    many_till((line, line_ending).map(|(line, _)| line), eof)
        .map(|(lines, _)| Chart { lines })
        .parse(input)
}

pub fn line(input: &str) -> IResult<&str, Line> {
    let res = alt((
        directive.map(Line::Directive),
        inline_content.map(|chunks| Line::Content {
            chunks,
            inline: true,
        }),
    ))
    .parse(input);
    res
}

pub fn directive(input: &str) -> IResult<&str, Directive> {
    (tag::<_, _, Error>("{"), take_until("}"), tag("}"))
        .map(|(_, content, _)| {
            match content.split_once(':') {
                Some(("title", title)) => return Directive::Title(title.to_owned()),
                Some(("comment", comment)) => return Directive::Comment(comment.to_owned()),
                Some(("key", key)) => {
                    if let Ok(key) = key.parse() {
                        return Directive::Key(key);
                    }
                }
                Some(("tempo", tempo)) => {
                    if let Ok(tempo) = tempo.trim().parse() {
                        return Directive::Tempo(tempo);
                    }
                }
                _ => {}
            };
            Directive::Other(content.to_owned())
        })
        .parse(input)
}

pub fn inline_content(input: &str) -> IResult<&str, Vec<Chunk>> {
    many0(chunk).parse(input)
}

pub fn chunk(input: &str) -> IResult<&str, Chunk> {
    (
        opt(boxed_chord),
        take_while1(|c: char| c != '[' && !c.is_newline()),
    )
        .map(|(chord, lyrics)| Chunk {
            chord,
            lyrics: lyrics.to_owned(),
        })
        .parse(input)
}

pub fn boxed_chord(input: &str) -> IResult<&str, Chord> {
    (tag("["), chord, tag("]"))
        .map(|(_, chord, _)| chord)
        .parse(input)
}

pub fn chord(input: &str) -> IResult<&str, Chord> {
    (letter_note, chord_quality)
        .map(|(note, quality)| Chord(note, quality))
        .parse(input)
}

pub fn chord_quality(input: &str) -> IResult<&str, ChordQuality> {
    alphanumeric0
        .map(|s: &str| ChordQuality(s.to_owned()))
        .parse(input)
}

pub fn scale(input: &str) -> IResult<&str, Scale> {
    letter_note.map(Scale).parse(input)
}

pub fn letter_note(input: &str) -> IResult<&str, LetterNote> {
    (letter, accidental)
        .map(|(l, a)| LetterNote(l, a))
        .parse(input)
}

pub fn letter(input: &str) -> IResult<&str, Letter> {
    one_of("CDEFGAB")
        .map(|c| match c {
            'C' => Letter::C,
            'D' => Letter::D,
            'E' => Letter::E,
            'F' => Letter::F,
            'G' => Letter::G,
            'A' => Letter::A,
            'B' => Letter::B,
            _ => unreachable!(),
        })
        .parse(input)
}

pub fn accidental(input: &str) -> IResult<&str, Accidental> {
    alt((
        tag("bb").map(|_| Accidental::DOUBLE_FLAT),
        tag("b").map(|_| Accidental::FLAT),
        tag("##").map(|_| Accidental::DOUBLE_SHARP),
        tag("#").map(|_| Accidental::SHARP),
        success(Accidental::NATURAL),
    ))
    .parse(input)
}

impl FromStr for Chart {
    type Err = OwnedError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        complete(chart)
            .parse(input)
            .map(|(_, c)| c)
            .map_err(|e| e.to_owned())
    }
}

impl FromStr for Scale {
    type Err = OwnedError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        complete(scale)
            .parse(input)
            .map(|(_, s)| s)
            .map_err(|e| e.to_owned())
    }
}

impl FromStr for LetterNote {
    type Err = OwnedError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        complete(letter_note)
            .parse(input)
            .map(|(_, n)| n)
            .map_err(|e| e.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        charts::{Chunk, Line},
        directives::Directive,
        notes::{Accidental, Letter, LetterNote},
        parser::{chart, directive},
        scales::Scale,
    };

    use Letter::*;
    use nom::Parser;

    const DOUBLE_FLAT: Accidental = Accidental::DOUBLE_FLAT;
    const FLAT: Accidental = Accidental::FLAT;
    const NATURAL: Accidental = Accidental::NATURAL;
    const SHARP: Accidental = Accidental::SHARP;
    const DOUBLE_SHARP: Accidental = Accidental::DOUBLE_SHARP;

    const HOW_GREAT_THOU_ART: &str =
        include_str!("../examples/How-Great-Thou-Art-(Whakaaria-Mai).chordpro");

    #[test]
    fn test_parse_score() {
        let chart = chart.parse(HOW_GREAT_THOU_ART).unwrap().1;

        assert_eq!(chart.lines.len(), 34);
        assert_eq!(
            chart.lines[0],
            Line::Directive(Directive::Title(
                "How Great Thou Art (Whakaaria Mai)".to_owned()
            ))
        );
        assert_eq!(
            chart.lines[5],
            Line::Content {
                chunks: vec![],
                inline: true
            }
        );
        assert_eq!(
            chart.lines[6],
            Line::Content {
                chunks: vec![Chunk {
                    chord: None,
                    lyrics: "English:".to_owned()
                }],
                inline: true
            }
        );
        assert_eq!(
            chart.lines[7],
            Line::Content {
                chunks: vec![
                    Chunk {
                        chord: None,
                        lyrics: "Then sings my ".to_owned()
                    },
                    Chunk {
                        chord: Some(B.flat().major_chord()),
                        lyrics: "soul".to_owned()
                    }
                ],
                inline: true
            }
        );
        assert_eq!(
            chart.lines[9],
            Line::Content {
                chunks: vec![
                    Chunk {
                        chord: Some(G.natural().minor_chord()),
                        lyrics: "How great thou ".to_owned()
                    },
                    Chunk {
                        chord: Some(F.natural().major_chord()),
                        lyrics: "art".to_owned()
                    }
                ],
                inline: true
            }
        );
    }

    #[test]
    fn test_parse_directives() {
        let directives = HOW_GREAT_THOU_ART
            .lines()
            .take(5)
            .map(|input| directive(input).unwrap().1)
            .collect::<Vec<_>>();

        assert_eq!(
            directives,
            vec![
                Directive::Title("How Great Thou Art (Whakaaria Mai)".to_owned()),
                Directive::Comment(
                    "Arrangement: Female Key (Db)  Male Key (Bb)  -  76bpm".to_owned()
                ),
                Directive::Key(Scale(LetterNote(B, FLAT))),
                Directive::Tempo(76),
                Directive::Other("ccli:7195204".to_owned()),
            ]
        );
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
