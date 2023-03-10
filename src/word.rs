use crate::{state::{Data, Stack}, context::Context};


const WORD_ADD: &str = "+";
const WORD_SUB: &str = "-";
const WORD_MUL: &str = "*";
const WORD_DIV: &str = "/";
const WORD_ABS: &str = "abs";

const WORD_DUP: &str = "dup";
const WORD_DROP: &str = "drop";
const WORD_PRINT: &str = "print";

const WORD_CMP: &str = "=";
const MODIFIER_COND: &str = "?";
const MODIFIER_LOOP: &str = "@";


pub enum Word {
    // Arythmetic operators

    Add,
    Sub,
    Mul,
    Div,
    Abs,

    // Stack operators

    /// Duplicate the top element of the stack.
    Dup,
    /// Drop the top element of the stack.
    Drop,
    /// Print the top element of the stack without dropping it.
    Print,

    // Branching

    /// Comparison operator.
    /// Takes 2 values from the stack.
    /// Puts on the stack:
    /// - `-1` if the top element is less than the second element;
    /// - `0` if they are equal;
    /// - `1` if the top element is greater than the second element.
    Cmp,
    /// Conditional execution.
    /// Takes 1 value from the stack and executes the corresponding word if it's not 0.
    Cond { then: Box<Word> },
    /// Loop execution.
    /// Takes 1 value from the stack and executes the corresponding word that many times.
    /// Before each iteration, puts the number of the current iteration on the stack, starting at 0 and ending at the value taken from the stack - 1.
    Loop { body: Box<Word> },

    // Other

    /// Reference to a compound word
    Ref { to: String },

    /// Constant
    Const { value: Data },
}


impl Word {
    pub fn try_parse(input: &str, context: &Context) -> Option<Word> {
        // Take care of conditional execution
        if input.ends_with(MODIFIER_COND) {
            let inner = input.trim_end_matches(MODIFIER_COND);

            if let Some(inner) = Word::try_parse(inner, context) {
                return Some(Self::Cond { then: Box::new(inner) });
            }
        }

        // Take care of loop execution
        if input.ends_with(MODIFIER_LOOP) {
            let inner = input.trim_end_matches(MODIFIER_LOOP);

            if let Some(inner) = Word::try_parse(inner, context) {
                return Some(Self::Loop { body: Box::new(inner) });
            }
        }

        match input {
            WORD_ADD => Some(Self::Add),
            WORD_SUB => Some(Self::Sub),
            WORD_MUL => Some(Self::Mul),
            WORD_DIV => Some(Self::Div),
            WORD_ABS => Some(Self::Abs),

            WORD_DUP => Some(Self::Dup),
            WORD_DROP => Some(Self::Drop),
            WORD_PRINT => Some(Self::Print),

            WORD_CMP => Some(Self::Cmp),

            _ => {
                // Check if we have a compound word with this name
                if let Some(_) = context.get(input) {
                    Some(Self::Ref { to: input.to_string() })
                } else {
                    // Otherwise, try to interpret it as a constant
                    Some(Self::Const {
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
            Self::Abs => stack.op1(|a| Data::new(a.value().abs())),

            Self::Dup => stack.dup(),
            Self::Drop => { stack.pop().unwrap(); () },
            Self::Print => println!("=> {}", stack.peek().unwrap().value()),

            Self::Cmp => stack.op2(|a, b| Data::new(- (b.value() - a.value()).signum())),
            Self::Cond { then } => {
                let condition = stack.pop().unwrap();
                if condition.value() != 0 {
                    then.execute(stack, context);
                }
            },
            Self::Loop { body } => {
                let iterations = stack.pop().unwrap().value();
                for i in 0..iterations {
                    stack.push(Data::new(i));
                    body.execute(stack, context);
                }
            },

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
