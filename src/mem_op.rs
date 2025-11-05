use curve25519_dalek::scalar::Scalar;

#[derive(Clone, Debug, PartialEq)]
pub struct MemOp {
    pub addr: Scalar,
    pub time: Scalar,
    pub is_store: bool, // 1 = store, 0 = load
    pub value: Scalar,
}

impl MemOp {
    pub fn new(addr: Scalar, time: Scalar, is_store: bool, value: Scalar) -> Self {
        Self {
            addr,
            time,
            is_store,
            value,
        }
    }
}
