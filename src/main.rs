//! Entry point for our program
mod cpu;
pub mod utils;
pub use utils::*;
use cpu::*;

fn main() {
    let my_cpu = CPU{ registers: RegisterFile::Zero(8)};
    println!("The cpu is: {:?}", my_cpu.ldr());
}
