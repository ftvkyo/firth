#[derive(Clone)]
pub struct Data(i32);

impl Data {
    pub fn new(value: i32) -> Data {
        Data(value)
    }

    pub fn value(&self) -> i32 {
        self.0
    }
}


pub struct Stack {
    data: Vec<Data>,
}

impl Stack {
    pub fn new() -> Stack {
        Stack { data: Vec::new() }
    }

    pub fn push(&mut self, value: Data) {
        self.data.push(value);
    }

    pub fn pop(&mut self) -> Option<Data> {
        self.data.pop()
    }

    pub fn dup(&mut self) {
        self.push(self.data.last().unwrap().clone());
    }

    pub fn peek(&self) -> Option<&Data> {
        self.data.last()
    }

    pub fn op1(&mut self, f: impl FnOnce(Data) -> Data) {
        let a = self.pop().unwrap();
        self.push(f(a));
    }

    pub fn op2(&mut self, f: impl FnOnce(Data, Data) -> Data) {
        let a = self.pop().unwrap();
        let b = self.pop().unwrap();
        self.push(f(a, b));
    }
}
