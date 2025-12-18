use std::fmt;

use crate::scales::Scale;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Directive {
    Title(String),
    Comment(String),
    Key(Scale),
    Tempo(u32),
    Other(String),
}

impl fmt::Display for Directive {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Directive::Title(title) => write!(f, "{{title:{title}}}"),
            Directive::Comment(comment) => write!(f, "{{comment:{comment}}}"),
            Directive::Key(scale) => write!(f, "{{key:{scale}}}"),
            Directive::Tempo(tempo) => write!(f, "{{tempo:{tempo}}}"),
            Directive::Other(content) => write!(f, "{{{content}}}"),
        }
    }
}
