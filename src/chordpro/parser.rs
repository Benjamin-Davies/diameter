use std::{cell::Cell, str::FromStr};

use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{tag, take_until, take_while, take_while1},
    character::complete::{line_ending, one_of, space0, space1},
    combinator::{eof, opt, success},
    multi::{many_till, many0, separated_list1},
};

use crate::{
    chordpro::{
        charts::{Chart, Chunk, Line},
        directives::Directive,
    },
    theory::{
        chords::{Chord, ChordQuality},
        notes::{Accidental, Letter, LetterNote, Note},
        scales::{Scale, ScaleDegree},
    },
};

type Span<'input> = nom_locate::LocatedSpan<&'input str>;
type Error<'input> = nom::error::Error<Span<'input>>;

thread_local! {
    static EXTENSIONS_ENABLED: Cell<bool> = Cell::new(false);
}

/// Enables or disables extensions **for the current thread**.
pub fn set_extensions_enabled(enabled: bool) {
    EXTENSIONS_ENABLED.with(|cell| cell.set(enabled));
}

fn chart(input: Span) -> IResult<Span, Chart> {
    many_till((line, opt(line_ending)).map(|(line, _)| line), eof)
        .map(|(lines, _)| Chart { lines })
        .parse(input)
}

fn line(input: Span) -> IResult<Span, Line> {
    alt((
        directive.map(Line::Directive),
        chords_over_lyrics_content.map(|chunks| Line::Content {
            chunks,
            inline: false,
        }),
        inline_content.map(|chunks| Line::Content {
            chunks,
            inline: true,
        }),
    ))
    .parse(input)
}

fn directive(input: Span) -> IResult<Span, Directive> {
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
            Directive::Other((*content).to_owned())
        })
        .parse(input)
}

fn chords_over_lyrics_content<'a>(input: Span<'a>) -> IResult<Span<'a>, Vec<Chunk>> {
    let extensions_enabled = EXTENSIONS_ENABLED.with(|cell| cell.get());
    if !extensions_enabled {
        return Err(nom::Err::Error(Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    }

    let start_len = input.len();
    (
        space0,
        separated_list1(space1, |input: Span<'a>| {
            let index = start_len - input.len();
            alt((boxed_chord, chord))
                .map(|chord| (index, chord))
                .parse(input)
        }),
        space0,
        alt((
            eof.map(|_| ""),
            (line_ending, eof).map(|(_, _)| ""),
            (
                line_ending,
                take_while::<_, Span, Error>(|c| c != '\r' && c != '\n'),
            )
                .map::<_, &str>(|(_, s)| &*s),
        )),
    )
        .map(|(_, chords, _, lyrics)| {
            let mut chunks = Vec::new();
            if chords[0].0 != 0 {
                let index = chords[0].0.min(lyrics.len());
                chunks.push(Chunk {
                    chord: None,
                    lyrics: lyrics[..index].to_owned(),
                });
            }
            for (i, (start_index, chord)) in chords.iter().enumerate() {
                let start_index = (*start_index).min(lyrics.len());
                let end_index = chords
                    .get(i + 1)
                    .map_or(usize::MAX, |&(next_index, _)| next_index)
                    .min(lyrics.len());
                chunks.push(Chunk {
                    chord: Some(chord.clone()),
                    lyrics: lyrics[start_index..end_index].to_owned(),
                });
            }
            chunks
        })
        .parse(input)
}

fn inline_content(input: Span) -> IResult<Span, Vec<Chunk>> {
    many0(chunk).parse(input)
}

fn is_lyrics_char(c: char) -> bool {
    c != '[' && c != '\r' && c != '\n'
}

fn chunk(input: Span) -> IResult<Span, Chunk> {
    alt((
        (boxed_chord, take_while(is_lyrics_char)).map(|(chord, lyrics)| Chunk {
            chord: Some(chord),
            lyrics: (*lyrics).to_owned(),
        }),
        (take_while1(is_lyrics_char)).map(|lyrics: Span| Chunk {
            chord: None,
            lyrics: (*lyrics).to_owned(),
        }),
    ))
    .parse(input)
}

fn boxed_chord(input: Span) -> IResult<Span, Chord> {
    (tag("["), chord, tag("]"))
        .map(|(_, chord, _)| chord)
        .parse(input)
}

fn chord(input: Span) -> IResult<Span, Chord> {
    (note, chord_quality, opt((tag("/"), note).map(|(_, b)| b)))
        .map(|(root, quality, bass)| Chord {
            root,
            quality,
            bass,
        })
        .parse(input)
}

fn chord_quality(input: Span) -> IResult<Span, ChordQuality> {
    take_while(|c: char| c.is_digit(10) || "Majminsusadd+-".contains(c))
        .map(|s: Span| ChordQuality((*s).to_owned()))
        .parse(input)
}

fn scale(input: Span) -> IResult<Span, Scale> {
    letter_note.map(Scale).parse(input)
}

fn note(input: Span) -> IResult<Span, Note> {
    alt((
        letter_note.map(Note::Letter),
        scale_degree.map(Note::Number),
    ))
    .parse(input)
}

fn letter_note(input: Span) -> IResult<Span, LetterNote> {
    (letter, accidental)
        .map(|(l, a)| LetterNote(l, a))
        .parse(input)
}

fn letter(input: Span) -> IResult<Span, Letter> {
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

fn scale_degree(input: Span) -> IResult<Span, ScaleDegree> {
    (accidental, bare_scale_degree)
        .map(|(accidental, degree)| ScaleDegree::new(degree, accidental))
        .parse(input)
}

fn bare_scale_degree(input: Span) -> IResult<Span, u8> {
    one_of("1234567")
        .map(|c| c.to_digit(10).unwrap() as u8)
        .parse(input)
}

fn accidental(input: Span) -> IResult<Span, Accidental> {
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
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        chart
            .parse(Span::new(input))
            .map(|(_, c)| c)
            .map_err(|e| e.to_string())
    }
}

impl FromStr for Scale {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        scale
            .parse(Span::new(input))
            .map(|(_, s)| s)
            .map_err(|e| e.to_string())
    }
}

impl FromStr for LetterNote {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        letter_note
            .parse(Span::new(input))
            .map(|(_, n)| n)
            .map_err(|e| e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        chordpro::{
            charts::{Chart, Chunk, Line},
            directives::Directive,
            parser::{Span, directive, set_extensions_enabled},
        },
        theory::{
            chords::Chord,
            notes::{Accidental, Letter, LetterNote},
            scales::Scale,
        },
    };

    use Letter::*;

    const DOUBLE_FLAT: Accidental = Accidental::DOUBLE_FLAT;
    const FLAT: Accidental = Accidental::FLAT;
    const NATURAL: Accidental = Accidental::NATURAL;
    const SHARP: Accidental = Accidental::SHARP;
    const DOUBLE_SHARP: Accidental = Accidental::DOUBLE_SHARP;

    const CHROMATIC_RUN: &str = include_str!("../../examples/Chromatic-Run.chordpro");
    const HOW_GREAT_THOU_ART: &str =
        include_str!("../../examples/How-Great-Thou-Art-(Whakaaria-Mai).chordpro");
    const O_HOLY_NIGHT: &str = include_str!("../../examples/O-Holy-Night-.chordpro");
    const TRAILING_CHORDS: &str = include_str!("../../examples/Trailing-Chords.chordpro");

    #[test]
    fn test_parse_inline_chart() {
        set_extensions_enabled(false);
        let chart = HOW_GREAT_THOU_ART.parse::<Chart>().unwrap();

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
    fn test_parse_inline_chart_extensions_enabled() {
        set_extensions_enabled(true);
        let chart = HOW_GREAT_THOU_ART.parse::<Chart>().unwrap();

        set_extensions_enabled(false);
        let chart_without_extensions = HOW_GREAT_THOU_ART.parse::<Chart>().unwrap();

        assert_eq!(chart, chart_without_extensions);
    }

    #[test]
    fn test_parse_over_lyrics_chart() {
        set_extensions_enabled(true);
        let chart = O_HOLY_NIGHT.parse::<Chart>().unwrap();

        assert_eq!(chart.lines.len(), 55);
        assert_eq!(
            chart.lines[0],
            Line::Directive(Directive::Title("O Holy Night ".to_owned()))
        );
        assert_eq!(
            chart.lines[9],
            Line::Content {
                chunks: vec![Chunk {
                    chord: None,
                    lyrics: "Intro".to_owned()
                }],
                inline: true
            }
        );
        assert_eq!(
            chart.lines[10],
            Line::Content {
                chunks: vec![
                    Chunk {
                        chord: Some(G.natural().major_chord()),
                        lyrics: " ".to_owned()
                    },
                    Chunk {
                        chord: Some(D.natural().major_chord()),
                        lyrics: "".to_owned()
                    },
                    Chunk {
                        chord: Some(E.natural().minor_chord()),
                        lyrics: "".to_owned()
                    },
                    Chunk {
                        chord: Some(C.natural().major_chord()),
                        lyrics: "".to_owned()
                    },
                ],
                inline: false
            }
        );
        assert_eq!(
            chart.lines[13],
            Line::Content {
                chunks: vec![
                    Chunk {
                        chord: Some(G.natural().major_chord()),
                        lyrics: "O holy ".to_owned()
                    },
                    Chunk {
                        chord: Some(D.natural().major_chord()),
                        lyrics: "night the ".to_owned()
                    },
                    Chunk {
                        chord: Some(C.natural().major_chord()),
                        lyrics: "stars are brightly s".to_owned()
                    },
                    Chunk {
                        chord: Some(E.natural().minor_chord()),
                        lyrics: "hining".to_owned()
                    },
                ],
                inline: false
            }
        );
        assert_eq!(
            chart.lines[21],
            Line::Content {
                chunks: vec![Chunk {
                    chord: None,
                    lyrics: "Chorus 1 ".to_owned()
                }],
                inline: true
            }
        );
        assert_eq!(
            chart.lines[chart.lines.len() - 1],
            Line::Content {
                chunks: vec![
                    Chunk {
                        chord: Some(G.natural().major_chord()),
                        lyrics: "".to_owned()
                    },
                    Chunk {
                        chord: Some(D.natural().major_chord()),
                        lyrics: "".to_owned()
                    },
                    Chunk {
                        chord: Some(E.natural().minor_chord()),
                        lyrics: "".to_owned()
                    },
                    Chunk {
                        chord: Some(C.natural().major_chord()),
                        lyrics: "".to_owned()
                    },
                ],
                inline: false
            }
        );
    }

    #[test]
    fn test_parse_over_lyric_extensions_disabled() {
        set_extensions_enabled(false);
        let chart = O_HOLY_NIGHT.parse::<Chart>().unwrap();

        assert_eq!(chart.lines.len(), 72);
    }

    #[test]
    fn test_parse_numbers() {
        set_extensions_enabled(true);
        let chart = CHROMATIC_RUN.parse::<Chart>().unwrap();

        assert_eq!(chart.lines.len(), 5);
        assert_eq!(
            chart.lines[0],
            Line::Directive(Directive::Title("Chromatic Run".to_owned()))
        );
        assert_eq!(
            chart.lines[4],
            Line::Content {
                chunks: vec![
                    Chunk {
                        chord: Some(Chord::major(1)),
                        lyrics: "".to_owned()
                    },
                    Chunk {
                        chord: Some(Chord::major(1).over(3)),
                        lyrics: "".to_owned()
                    },
                    Chunk {
                        chord: Some(Chord::major(1).over(4)),
                        lyrics: "".to_owned()
                    },
                    Chunk {
                        chord: Some(Chord::major(1).over((4, SHARP))),
                        lyrics: "".to_owned()
                    },
                    Chunk {
                        chord: Some(Chord::major(1).over(5)),
                        lyrics: "".to_owned()
                    },
                    Chunk {
                        chord: Some(Chord::major(1).over(6)),
                        lyrics: "".to_owned()
                    },
                    Chunk {
                        chord: Some(Chord::major(1).over((7, FLAT))),
                        lyrics: "".to_owned()
                    },
                    Chunk {
                        chord: Some(Chord::major(1).over(7)),
                        lyrics: "".to_owned()
                    },
                ],
                inline: false
            }
        );
    }

    #[test]
    fn test_parse_trailing_chords() {
        set_extensions_enabled(true);
        let chart = TRAILING_CHORDS.parse::<Chart>().unwrap();

        assert_eq!(chart.lines.len(), 1);
        assert_eq!(
            chart.lines[0],
            Line::Content {
                chunks: vec![
                    Chunk {
                        chord: Some(Chord::major(1)),
                        lyrics: "Lorem ".to_owned()
                    },
                    Chunk {
                        chord: Some(Chord::minor(2)),
                        lyrics: "ipsum ".to_owned()
                    },
                    Chunk {
                        chord: Some(Chord::major(1).over(3)),
                        lyrics: "dolor ".to_owned()
                    },
                    Chunk {
                        chord: Some(Chord::major(4)),
                        lyrics: "sit ".to_owned()
                    },
                    Chunk {
                        chord: Some(Chord::major(5)),
                        lyrics: "amet ".to_owned()
                    },
                    Chunk {
                        chord: Some(Chord::minor(6)),
                        lyrics: " ".to_owned()
                    },
                    Chunk {
                        chord: Some(Chord::major(5).over(7)),
                        lyrics: "".to_owned()
                    },
                    Chunk {
                        chord: Some(Chord::major(1)),
                        lyrics: "".to_owned()
                    }
                ],
                inline: true
            }
        );
    }

    #[test]
    fn test_parse_directives() {
        set_extensions_enabled(false);
        let directives = HOW_GREAT_THOU_ART
            .lines()
            .take(5)
            .map(|input| directive(Span::new(input)).unwrap().1)
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
