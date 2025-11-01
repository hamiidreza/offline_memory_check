use crate::mem_op::MemOp;
use curve25519_dalek::scalar::Scalar;

#[derive(Clone, Debug, PartialEq)]
pub struct Transcript {
    pub ops: Vec<MemOp>,
}

impl Transcript {
    pub fn new() -> Self {
        Self { ops: Vec::new() }
    }

    pub fn push(&mut self, op: MemOp) {
        self.ops.push(op);
    }

    pub fn sort_by_addr_time(&self) -> Self {
        let mut sorted_ops = self.ops.clone();
        sorted_ops.sort_by(|a, b| {
            a.addr
                .partial_cmp(&b.addr)
                .unwrap()
                .then(a.time.partial_cmp(&b.time).unwrap())
        });
        Self { ops: sorted_ops }
    }

    pub fn check_memory_consistency(&self) -> ConsistencyReport {
        let mut current_addr: Option<u64> = None;
        let mut current_val: Option<Scalar> = None;

        for (i, op) in self.ops.iter().enumerate() {
            if current_addr.is_none() || current_addr != Some(op.addr) {
                // New address, reset current value
                current_addr = Some(op.addr);
                current_val = None;
            }

            if op.is_store == true {
                // Store operation
                current_val = Some(op.value);
            } else {
                // Load operation
                if let Some(expected_val) = &current_val {
                    if *expected_val != op.value {
                        return ConsistencyReport {
                            consistent: false,
                            first_bad_index: Some(i),
                            expected: Some(*expected_val),
                            actual: Some(op.value),
                        };
                    }
                    else{}
                }
            }
        }

        return ConsistencyReport {
            consistent: true,
            first_bad_index: None,
            expected: None,
            actual: None,
        };
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConsistencyReport {
    pub consistent: bool,
    pub first_bad_index: Option<usize>,
    pub expected: Option<Scalar>,
    pub actual: Option<Scalar>,
}