use std::str::SplitAsciiWhitespace;

use crate::parser::{Expression, Word};

pub enum EvaluationError {}

pub enum SideEffect<'a> {
    Print(&'a str),
}

pub struct State<'a> {
    stack: Vec<Word<'a>>,
}

impl State<'_> {
    pub fn evaluate(&mut self, expr: Expression) -> Result<Vec<SideEffect>, EvaluationError> {
        let mut side_effects = vec![];
        for word in expr {
            if let Some(se) = self.apply_word(word)? {
                side_effects.push(se)
            }
        }
        return Ok(side_effects);
    }

    fn apply_word<'b>(
        &mut self,
        word: Word<'_>,
    ) -> Result<Option<SideEffect<'b>>, EvaluationError> {
        return Ok(None);
    }
}
