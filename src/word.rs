use crate::state::{Data, Stack};


const WORD_ADD: &str = "+";
const WORD_SUB: &str = "-";
const WORD_PRINT: &str = ".";
const WORD_DEBUG_INDICATOR: &str = "#";


pub struct Word {
    kind: Box<dyn WordKind>,
}

impl Word {
    pub fn try_parse(input: &str) -> Option<Word> {
        let kind: Box<dyn WordKind> = match input {
            WORD_ADD => Box::new(Add),
            WORD_SUB => Box::new(Sub),
            WORD_PRINT => Box::new(Print),
            x => {
                if x.starts_with(WORD_DEBUG_INDICATOR) {
                    let action = match &x[WORD_DEBUG_INDICATOR.len()..] {
                        "print" => DebugAction::StackPrint,
                        "clear" => DebugAction::StackClear,
                        _ => return None,
                    };
                    Box::new(Debug { action })
                } else {
                    return None;
                }
            },
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


pub struct Print;

impl WordKind for Print {
    fn execute(&self, stack: &mut Stack) {
        let a = stack.pop().unwrap();
        println!("=> {}", a.value());
    }
}


enum DebugAction {
    StackPrint,
    StackClear,
}


pub struct Debug {
    action: DebugAction,
}

impl WordKind for Debug {
    fn execute(&self, stack: &mut Stack) {
        match self.action {
            DebugAction::StackPrint => {
                print!("=> Stack: [");
                for data in stack.get().iter() {
                    print!("{}, ", data.value());
                }
                println!("]");
            }
            DebugAction::StackClear => {
                stack.clear();
            }
        }
    }
}
