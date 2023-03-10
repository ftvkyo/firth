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
}
