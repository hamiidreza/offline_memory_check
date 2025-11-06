use curve25519_dalek::scalar::Scalar;
use offline_memory_check::mem_op::MemOp;
use offline_memory_check::transcript::Transcript;

#[test]
fn consistency_check_ok_test() {
    let addr = Scalar::from(5u64);
    let t1 = Scalar::from(1u64);
    let v1 = Scalar::from(9u64);
    let t2 = Scalar::from(2u64);

    let mut tr = Transcript::new();
    tr.push(MemOp::new(addr, t1, true, v1)); // store 9 at addr 5 at time 1
    tr.push(MemOp::new(addr, t2, false, v1)); // load 9 at addr 5 at time 2

    let report = tr.sort_by_addr_time().check_memory_consistency();
    assert!(report.consistent, "expected consistent trace: {report:?}");
}

#[test]
fn consistency_check_error_test() {
    let addr = Scalar::from(5u64);
    let t1 = Scalar::from(1u64);
    let v1 = Scalar::from(9u64);
    let t2 = Scalar::from(2u64);
    let v_wrong = Scalar::from(7u64);

    let mut tr = Transcript::new();
    tr.push(MemOp::new(addr, t1, true, v1)); // store 9 at addr 5 at time 1
    tr.push(MemOp::new(addr, t2, false, v_wrong)); // load 7 at addr 5 at time 2

    let report = tr.sort_by_addr_time().check_memory_consistency();
    assert!(
        !report.consistent,
        "expected inconsistent trace: {report:?}"
    );
    assert_eq!(report.first_bad_index, Some(1));
    assert_eq!(report.expected, Some(v1));
    assert_eq!(report.actual, Some(v_wrong));
}
