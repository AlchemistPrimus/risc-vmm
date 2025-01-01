//! Entry point for our program
mod cpu;
pub mod utils;
pub use utils::cpu_utils::{Memory};
use cpu::{CPU};
use cpu::*;

fn main() {
    // Initializing an empty struct.
    // This struct implemented default attribute.
    // The CPU should be booted with some default values.
    let mut my_cpu = CPU::boot();
    let mut mem = Memory::new();
    mem.push_data(Box::new(8));
    //&my_cpu.boot();
    my_cpu.lw("t0", 10);
    my_cpu.lw("t1", 12);
    my_cpu.run();
    let res = my_cpu.sw(8);
    //&my_cpu.read_opcode(&mem);
    println!("The operation value {:?}", res);
}
