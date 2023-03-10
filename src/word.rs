use crate::{state::{Data, Stack}, context::Context};


const WORD_ADD: &str = "+";
const WORD_SUB: &str = "-";
const WORD_MUL: &str = "*";
const WORD_DIV: &str = "/";
const WORD_DUP: &str = "dup";


pub struct Word {
    kind: Box<dyn WordKind>,
}

impl Word {
    pub fn try_parse(input: &str, context: &Context) -> Option<Word> {
        let kind: Box<dyn WordKind> = match input {
            WORD_ADD => Box::new(Add),
            WORD_SUB => Box::new(Sub),
            WORD_MUL => Box::new(Mul),
            WORD_DIV => Box::new(Div),
            WORD_DUP => Box::new(Dup),
            _ => {
                // Check if we have a compound word with this name
                if let Some(_) = context.get(input) {
                    Box::new(Ref {to: input.to_string()})
                } else {
                    // Otherwise, try to interpret it as a constant
                    Box::new(Const::try_parse(input)?)
                }
            },
        };
        Some(Word { kind })
    }

    pub fn execute(&self, stack: &mut Stack, context: &Context) {
        self.kind.execute(stack, context);
    }
}


pub trait WordKind {
    fn execute(&self, stack: &mut Stack, context: &Context);
}


pub struct Add;

impl WordKind for Add {
    fn execute(&self, stack: &mut Stack, _context: &Context) {
        let a = stack.pop().unwrap();
        let b = stack.pop().unwrap();
        stack.push(Data::new(a.value() + b.value()));
    }
}


pub struct Sub;

impl WordKind for Sub {
    fn execute(&self, stack: &mut Stack, _context: &Context) {
        let a = stack.pop().unwrap();
        let b = stack.pop().unwrap();
        // B - A because the stack is LIFO
        stack.push(Data::new(b.value() - a.value()));
    }
}


pub struct Mul;

impl WordKind for Mul {
    fn execute(&self, stack: &mut Stack, _context: &Context) {
        let a = stack.pop().unwrap();
        let b = stack.pop().unwrap();
        stack.push(Data::new(a.value() * b.value()));
    }
}


pub struct Div;

impl WordKind for Div {
    fn execute(&self, stack: &mut Stack, _context: &Context) {
        let a = stack.pop().unwrap();
        let b = stack.pop().unwrap();
        stack.push(Data::new(b.value() / a.value()));
    }
}


pub struct Dup;

impl WordKind for Dup {
    fn execute(&self, stack: &mut Stack, _context: &Context) {
        let a = stack.pop().unwrap();
        stack.push(a.clone());
        stack.push(a);
    }
}


pub struct Const {
    value: Data,
}

impl Const {
    pub fn try_parse(input: &str) -> Option<Const> {
        let value = input.parse::<i32>().ok().map(|x| Data::new(x))?;
        Some(Const {
            value
        })
    }
}

impl WordKind for Const {
    fn execute(&self, stack: &mut Stack, _context: &Context) {
        stack.push(self.value.clone());
    }
}


pub struct Ref {
    to: String,
}

impl WordKind for Ref {
    fn execute(&self, stack: &mut Stack, context: &Context) {
        let sequence = context.get(&self.to).unwrap();
        for word in sequence {
            word.execute(stack, context);
        }
    }
}
