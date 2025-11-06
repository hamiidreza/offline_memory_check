use curve25519_dalek::Scalar;
use offline_memory_check::mem_op::MemOp;
use offline_memory_check::transcript::Transcript;

fn main() {
    let mut tr = Transcript::new();
    tr.push(MemOp::new(
        Scalar::from(5u64),
        Scalar::from(1u64),
        true,
        Scalar::from(9u64),
    ));
    tr.push(MemOp::new(
        Scalar::from(2u64),
        Scalar::from(2u64),
        true,
        Scalar::from(7u64),
    ));
    tr.push(MemOp::new(
        Scalar::from(5u64),
        Scalar::from(0u64),
        false,
        Scalar::from(0u64),
    ));
    let sorted_tr = tr.sort_by_addr_time();
    println!("{:?}", sorted_tr);
}
