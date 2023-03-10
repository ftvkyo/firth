pub mod context;
pub mod state;
pub mod word;


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

        let is_definition = line.starts_with(":");
        let line = line.strip_prefix(":").unwrap_or(line);

        let mut tokens = line.split_whitespace();
        let mut definition_name = None;

        if is_definition {
            definition_name = Some(tokens.next().unwrap());
        }

        let words = tokens
            .map(|token| word::Word::try_parse(token, &context))
            .map(Option::unwrap);

        if let Some(name) = definition_name {
            context.insert(name.to_string(), words.collect());
            println!("Defined {}", name);
            continue;
        }

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
