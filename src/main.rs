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
    loop {
        let line = inquire::Text::new("=> ")
            .prompt()
            .unwrap();

        if line == "exit" {
            break;
        }

        let tokens = line
            .split_whitespace()
            .map(Token::parse)
            .map(Result::unwrap);

        let mut stack = state::Stack::new();
        for token in tokens {
            match token {
                Token::Word(word) => word.execute(&mut stack),
                Token::Data(data) => stack.push(data),
            }
        }

        // If there's something left on the stack, print it
        while let Some(data) = stack.pop() {
            println!("{}", data.value());
        }
    }
}
