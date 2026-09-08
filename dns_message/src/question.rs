use bilge::prelude::*;
#[bitsize(16)]
#[derive(FromBits, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum QuestionType {
    // TODO
    #[fallback]
    Todo,
}

#[bitsize(16)]
#[derive(FromBits, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum QuestionClass {
    // TODO
    #[fallback]
    Todo,
}
