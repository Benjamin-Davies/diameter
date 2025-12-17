use crate::scales::Scale;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Directive {
    Title(String),
    Comment(String),
    Key(Scale),
    Tempo(u32),
    Other(String),
}
