use std::str::FromStr;

use anyhow::Context;

use crate::scales::Scale;

#[derive(Debug, PartialEq, Eq)]
pub enum Directive {
    Title(String),
    Comment(String),
    Key(Scale),
    Tempo(u32),
    Other(String),
}

impl FromStr for Directive {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let stripped = s
            .strip_prefix('{')
            .context("expected `{` in directive")?
            .strip_suffix('}')
            .context("expected `}` in directive")?;
        match stripped.split_once(':') {
            Some(("title", title)) => return Ok(Directive::Title(title.to_owned())),
            Some(("comment", comment)) => return Ok(Directive::Comment(comment.to_owned())),
            Some(("key", key)) => {
                if let Ok(key) = key.trim().parse() {
                    return Ok(Directive::Key(key));
                }
            }
            Some(("tempo", tempo)) => {
                if let Ok(tempo) = tempo.trim().parse() {
                    return Ok(Directive::Tempo(tempo));
                }
            }
            _ => {}
        }
        Ok(Directive::Other(stripped.to_owned()))
    }
}

#[cfg(test)]
mod tests {
    use crate::directives::Directive;

    const HOW_GREAT_THOU_ART: &str =
        include_str!("../examples/How-Great-Thou-Art-(Whakaaria-Mai).chordpro");

    #[test]
    fn test_parse_directives() {
        let directives = HOW_GREAT_THOU_ART
            .lines()
            .take(5)
            .map(|s| s.parse::<Directive>().unwrap())
            .collect::<Vec<_>>();

        assert_eq!(
            directives,
            vec![
                Directive::Title("How Great Thou Art (Whakaaria Mai)".to_owned()),
                Directive::Comment(
                    "Arrangement: Female Key (Db)  Male Key (Bb)  -  76bpm".to_owned()
                ),
                Directive::Key("Bb".parse().unwrap()),
                Directive::Tempo(76),
                Directive::Other("ccli:7195204".to_owned()),
            ]
        );
    }
}
