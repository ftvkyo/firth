use std::io::BufRead;

pub mod state;
pub mod word;


pub enum Token {
    Word(word::Word),
    Data(state::Data),
}

impl Token {
    pub fn parse(input: &str) -> Result<Token, ()> {
        if let Some(word) = word::Word::try_parse(input) {
            Ok(Token::Word(word))
        } else if let Some(data) = state::Data::try_parse(input) {
            Ok(Token::Data(data))
        } else {
            Err(())
        }
    }
}


fn main() {
    let mut stack = state::Stack::new();

    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let tokens = line.split_whitespace().map(Token::parse).map(Result::unwrap);

        for token in tokens {
            match token {
                Token::Word(word) => word.execute(&mut stack),
                Token::Data(data) => stack.push(data),
            }
        }
    }
}
