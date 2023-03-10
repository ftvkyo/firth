pub mod context;
pub mod state;
pub mod word;


fn tokens_to_words<'a>(tokens: impl Iterator<Item = &'a str>, context: &context::Context) -> Vec<word::Word> {
    tokens
        .map(|token| word::Word::try_parse(token, context))
        .map(Option::unwrap)
        .collect()
}


fn main() {
    let mut context = context::Context::new();

    loop {
        let input = inquire::Text::new("")
            .prompt()
            .unwrap();

        let line = input.trim();

        if line == "exit" {
            break;
        }

        let mut tokens = line.split_whitespace().peekable();
        let token1 = tokens.peek().unwrap();

        // Process a definition
        if *token1 == ":" {
            tokens.next(); // Skip the colon
            let name = tokens.next().unwrap();
            let words = tokens_to_words(tokens, &context);

            context.insert(name.to_string(), words);
            println!("Defined {}", name);
            continue;
        } else {
            // Otherwise it's an interactive line

            let words = tokens_to_words(tokens, &context);

            let mut stack = state::Stack::new();
            for word in words {
                word.execute(&mut stack, &context);
            }

            // If there's something left on the stack, print it
            while let Some(data) = stack.pop() {
                println!("{}", data.value());
            }
        }
    }
}
