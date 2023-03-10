use crate::state::{Data, Stack};


pub struct Word {
    kind: Box<dyn WordKind>,
}

impl Word {
    pub fn try_parse(input: &str) -> Option<Word> {
        let kind: Box<dyn WordKind> = match input {
            WORD_ADD => Box::new(Add),
            WORD_SUB => Box::new(Sub),
            WORD_PRINT => Box::new(Print),
            _ => return None,
        };
        Some(Word { kind })
    }

    pub fn execute(&self, stack: &mut Stack) {
        self.kind.execute(stack);
    }
}


pub trait WordKind {
    fn name(&self) -> &str;
    fn execute(&self, stack: &mut crate::state::Stack);
}


const WORD_ADD: &str = "+";
const WORD_SUB: &str = "-";
const WORD_PRINT: &str = ".";


pub struct Add;

impl WordKind for Add {
    fn name(&self) -> &str {
        WORD_ADD
    }

    fn execute(&self, stack: &mut Stack) {
        let a = stack.pop().unwrap();
        let b = stack.pop().unwrap();
        stack.push(Data::new(a.value() + b.value()));
    }
}


pub struct Sub;

impl WordKind for Sub {
    fn name(&self) -> &str {
        WORD_SUB
    }

    fn execute(&self, stack: &mut Stack) {
        let a = stack.pop().unwrap();
        let b = stack.pop().unwrap();
        // B - A because the stack is LIFO
        stack.push(Data::new(b.value() - a.value()));
    }
}


pub struct Print;

impl WordKind for Print {
    fn name(&self) -> &str {
        WORD_PRINT
    }

    fn execute(&self, stack: &mut Stack) {
        let a = stack.pop().unwrap();
        println!("=> {}", a.value());
    }
}
