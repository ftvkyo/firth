use crate::state::{Data, Stack};


const WORD_ADD: &str = "+";
const WORD_SUB: &str = "-";
const WORD_MUL: &str = "*";
const WORD_DIV: &str = "/";
const WORD_DUP: &str = "dup";


pub struct Word {
    kind: Box<dyn WordKind>,
}

impl Word {
    pub fn try_parse(input: &str) -> Option<Word> {
        let kind: Box<dyn WordKind> = match input {
            WORD_ADD => Box::new(Add),
            WORD_SUB => Box::new(Sub),
            WORD_MUL => Box::new(Mul),
            WORD_DIV => Box::new(Div),
            WORD_DUP => Box::new(Dup),
            _ => return None,
        };
        Some(Word { kind })
    }

    pub fn execute(&self, stack: &mut Stack) {
        self.kind.execute(stack);
    }
}


pub trait WordKind {
    fn execute(&self, stack: &mut crate::state::Stack);
}



pub struct Add;

impl WordKind for Add {
    fn execute(&self, stack: &mut Stack) {
        let a = stack.pop().unwrap();
        let b = stack.pop().unwrap();
        stack.push(Data::new(a.value() + b.value()));
    }
}


pub struct Sub;

impl WordKind for Sub {
    fn execute(&self, stack: &mut Stack) {
        let a = stack.pop().unwrap();
        let b = stack.pop().unwrap();
        // B - A because the stack is LIFO
        stack.push(Data::new(b.value() - a.value()));
    }
}


pub struct Mul;

impl WordKind for Mul {
    fn execute(&self, stack: &mut Stack) {
        let a = stack.pop().unwrap();
        let b = stack.pop().unwrap();
        stack.push(Data::new(a.value() * b.value()));
    }
}


pub struct Div;

impl WordKind for Div {
    fn execute(&self, stack: &mut Stack) {
        let a = stack.pop().unwrap();
        let b = stack.pop().unwrap();
        stack.push(Data::new(b.value() / a.value()));
    }
}


pub struct Dup;

impl WordKind for Dup {
    fn execute(&self, stack: &mut Stack) {
        let a = stack.pop().unwrap();
        stack.push(a.clone());
        stack.push(a);
    }
}
