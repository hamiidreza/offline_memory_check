use offline_memory_check::transcript::Transcript;
use offline_memory_check::mem_op::MemOp;

fn main(){
    let mut tr = Transcript::<u32>::new();
    tr.push(MemOp::new(5u32, 1u32, 1u32, 9u32));
    tr.push(MemOp::new(2u32, 2u32, 1u32, 7u32));
    tr.push(MemOp::new(5u32, 0u32, 0u32, 0u32));
    let sorted_tr = tr.sort_by_addr_time();
    println!("{:?}", sorted_tr);
}
