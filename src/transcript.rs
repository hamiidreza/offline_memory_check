use crate::mem_op::MemOp;

#[derive(Clone, Debug, PartialEq)]
pub struct Transcript<F> {
    pub ops: Vec<MemOp<F>>,
}

impl<F> Transcript<F> {
    pub fn new() -> Self {
        Self { ops: Vec::new() }
    }

    pub fn push(&mut self, op:MemOp<F>){
        self.ops.push(op);
    }
}

