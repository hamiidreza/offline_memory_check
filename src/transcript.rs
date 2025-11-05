use crate::mem_op::MemOp;
use curve25519_dalek::scalar::Scalar;
use core::cmp::Ordering;

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
            match cmp_scalar(&a.addr, &b.addr) {
                Ordering::Equal => cmp_scalar(&a.time, &b.time),
                other => other,
            }
        });
        Self { ops: sorted_ops }
    }

    pub fn check_memory_consistency(&self) -> ConsistencyReport {
        let mut current_addr: Option<Scalar> = None;
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

fn cmp_scalar(a: &Scalar, b: &Scalar) -> Ordering {
    let ab = a.to_bytes(); // [u8; 32]
    let bb = b.to_bytes();
    ab.cmp(&bb)
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConsistencyReport {
    pub consistent: bool,
    pub first_bad_index: Option<usize>,
    pub expected: Option<Scalar>,
    pub actual: Option<Scalar>,
}