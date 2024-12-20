//! CPU code

/// Contains the cpu registers.
/// Contains registers from register x0-x15 for RV32E.
#[derive(Copy, Clone, Debug)]
pub enum RegisterFile {
    Zero(u32),
    Ra(u32),
    Sp(u32),
    Gp(u32),
    Tp(u32),
    T0(u32),
    T1(u32),
    T2(u32),
    Fp(u32),
    S1(u32),
    A0(u32),
    A1(u32),
    A2(u32),
    A3(u32),
    A4(u32),
}


/// The cpu object.
/// Data is loaded on registers
/// Instructions can only be executed on the data on registers
#[derive(Copy, Clone, Debug)]
pub struct CPU {
    pub registers: RegisterFile,
}

impl CPU{
    pub fn ldr(&self) -> u32{
        // Access enum variant
        match self.registers {
            RegisterFile::Zero(val) => {val},
            _ => {0},
        }
    }
}
