#[derive(Debug, Clone)]
pub struct State {
    pub pc: usize,
    pub acc: i32,
    pub history: Vec<usize>,
}

impl State {
    #[allow(dead_code)]
    pub fn create(pc: usize, acc: i32, history: Vec<usize>) -> State {
        State { pc, acc, history }
    }

    pub fn new() -> Box<State> {
        Box::new(State {
            pc: 0,
            acc: 0,
            history: vec![],
        })
    }

    #[allow(dead_code)]
    pub fn inc_pc(&mut self, x: i32) -> &mut Self {
        self.pc = ((self.pc as i32) + x) as usize;
        self
    }

    #[allow(dead_code)]
    pub fn inc_acc(&mut self, x: i32) -> &mut Self {
        self.acc += x;
        self
    }

    #[allow(dead_code)]
    pub fn clear(&mut self) -> &mut Self {
        self.pc = 0;
        self.acc = 0;
        self.history.clear();
        self
    }
}
