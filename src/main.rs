use offline_memory_check::transcript::Transcript;
use offline_memory_check::mem_op::MemOp;

fn main(){
    let mut tr = Transcript::<u32>::new();
    tr.push(MemOp::new(1u32, 0u32, 1u32, 42u32));
    println!("{:?}", tr);
}
