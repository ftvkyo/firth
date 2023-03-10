use crate::{state::{Data, Stack}, context::Context};


const WORD_ADD: &str = "+";
const WORD_SUB: &str = "-";
const WORD_MUL: &str = "*";
const WORD_DIV: &str = "/";
const WORD_DUP: &str = "dup";


pub enum Word {
    Add,
    Sub,
    Mul,
    Div,
    Dup,
    Ref { to: String },
    Const { value: Data },
}


impl Word {
    pub fn try_parse(input: &str, context: &Context) -> Option<Word> {
        match input {
            WORD_ADD => Some(Self::Add),
            WORD_SUB => Some(Self::Sub),
            WORD_MUL => Some(Self::Mul),
            WORD_DIV => Some(Self::Div),
            WORD_DUP => Some(Self::Dup),
            _ => {
                // Check if we have a compound word with this name
                if let Some(_) = context.get(input) {
                    Some(Self::Ref { to: input.to_string() })
                } else {
                    // Otherwise, try to interpret it as a constant
                    Some(Self::Const{
                        value: input.parse::<i32>().ok().map(|x| Data::new(x))?
                    })
                }
            },
        }
    }

    pub fn execute(&self, stack: &mut Stack, context: &Context) {
        match self {
            Self::Add => stack.op2(|a, b| Data::new(b.value() + a.value())),
            Self::Sub => stack.op2(|a, b| Data::new(b.value() - a.value())),
            Self::Mul => stack.op2(|a, b| Data::new(b.value() * a.value())),
            Self::Div => stack.op2(|a, b| Data::new(b.value() / a.value())),
            Self::Dup => stack.dup(),
            Self::Ref { to } => {
                let words = context.get(to).unwrap();
                for word in words {
                    word.execute(stack, context);
                }
            },
            Self::Const { value } => stack.push(value.clone()),
        }
    }
}
