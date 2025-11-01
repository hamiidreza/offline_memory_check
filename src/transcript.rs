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

impl<F: Clone + PartialOrd> Transcript<F> {
    pub fn sort_by_addr_time(&self) -> Self {
        let mut sorted_ops = self.ops.clone();
        sorted_ops.sort_by(|a, b| {
            a.addr
                .partial_cmp(&b.addr)
                .unwrap()
                .then(a.time.partial_cmp(&b.time).unwrap())
        });
        Self { ops: sorted_ops}
    }
}

