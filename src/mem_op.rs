#[derive(Clone, Debug, PartialEq)]
pub struct MemOp<F> {
    pub addr: F,
    pub time: F,
    pub is_store: F,
    pub value: F,
}

impl<F> MemOp<F> {
    pub fn new(addr: F, time: F, is_store: F, value: F) -> Self {
        Self {
            addr,
            time,
            is_store,
            value,
        }
    }
}