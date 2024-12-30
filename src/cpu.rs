//! CPU code. RISC-V implementation.
//! RV32E specification.
use super::utils::cpu_utils::{Memory};

/// Contains the cpu registers.
/// Contains registers from register x0-x15 for RV32E.
#[derive(Clone, Debug)]
struct RegisterFile {
    general_purpose_registers: [u32; 16],
    program_counter: u32
}


/// The cpu object.
/// Data is loaded on registers
/// Instructions can only be executed on the data on registers
#[derive(Clone, Debug)]
pub struct CPU {
    registers: RegisterFile,
    memory: [u32; 0x1000],
}


/// Defining the methods supported by our cpu
pub trait Instructions {

    /// Creates a new CPU instance, and starting it up.
    fn boot() -> Self
        where Self: Sized;

    /// Load word from the memory, into registers.
    fn lw(&mut self, reg: &str, val: u32);

    /// Store word to the memory from the registers.
    fn sw(&self, reg_no: u32) -> u32;

    /// Add two values in the register.
    fn add(&mut self, reg1: u32, reg2: u32);

}

impl Instructions for CPU {
   fn boot() -> Self{
        // Creating a new cpu instance, initializing various values like the pc.
        println!("Started CPU process. Setting up the registers.");
        let r_file = RegisterFile{
            general_purpose_registers: [0; 16],
            program_counter: 0x00,
        };

        CPU {
            registers: r_file,
            memory: [0; 4096],
        }
    }

    fn add(&mut self, reg_no1: u32, reg_no2: u32) {
        // Add two operands.
        // This instruction works on temp registeres.
        // registers x5, x6, x7
        let first_reg = self.registers.general_purpose_registers[reg_no1 as usize];
        let second_reg = self.registers.general_purpose_registers[reg_no2 as usize];
        println!("The values to be added are {} {}", first_reg, second_reg);
        let (val, overflow_detected) = first_reg.overflowing_add(second_reg);
        println!("The added value is {}", val);
        self.registers.general_purpose_registers[reg_no1 as usize] = val;

        if overflow_detected {
            println!("An overflow has been detected");
        } else {
            println!("No overflow");
        }
    }

    fn lw(&mut self, reg: &str, val: u32){
        // Reads data word from memory into a register.
        // Takes the memory address number for the word, and target register.
        // Loads the data word into the register.

        // let instruction: u32 = self.read_opcode(&mem);
        // Decoding the opcode and determining the type of instruction.
        // let (instr_opcode, instr_body) = self.instruction_decoder(instruction); 

        // Interprate the opcode here for proper loading of data into the registers.
        
        // Loading data into the registers. Architectural registers corresponds 
        // one-for-one entries in the physical register file within the CPU.
        match reg {
            "ra" => { self.registers.general_purpose_registers[1 as usize] = val; },
            "sp" => { self.registers.general_purpose_registers[2 as usize] = val; },
            "gp" => { self.registers.general_purpose_registers[3 as usize] = val; },
            "tp" => { self.registers.general_purpose_registers[4 as usize] = val; },
            "t0" => { self.registers.general_purpose_registers[5 as usize] = val; },
            "t1" => { self.registers.general_purpose_registers[6 as usize] = val; },
            "t2" => { self.registers.general_purpose_registers[7 as usize] = val; },
            "s0" | "fp" => { self.registers.general_purpose_registers[8 as usize] = val;},
            "s1" => { self.registers.general_purpose_registers[9 as usize] = val; },
            "a0" => { self.registers.general_purpose_registers[10 as usize] = val; },
            "a1" => { self.registers.general_purpose_registers[11 as usize] = val; },
            "a2" => { self.registers.general_purpose_registers[12 as usize] = val; },
            "a3" => { self.registers.general_purpose_registers[13 as usize] = val; },
            "a4" => { self.registers.general_purpose_registers[14 as usize] = val; },
            "a5" => { self.registers.general_purpose_registers[15 as usize] = val; },
            _ => { panic!("No such register"); },
        }

    }

    fn sw(&self, reg_no: u32) -> u32{
        self.registers.general_purpose_registers[reg_no as usize]
    }

}

impl CPU{

    fn read_opcode(&mut self, mem: &Memory) -> u32{
        // Reads the opcode from the location of the memory address stored in the pc
        let opcode = &mem.data[self.registers.program_counter as usize];
        **opcode
    }


    pub fn run(&mut self) {
        // Execution the CPU instructions
        self.add(5, 6);
        println!("Th value saved on x1 register is {}", self.registers.general_purpose_registers[5 as usize]);
    }

    fn instruction_decoder(instr: u32) -> (String, u32){
        // Splits up the the instruction from its body and decodes the opcode_grp.
        // The type of instruction is determined here. Denoted as opcode_grp. 
        let opcode_grp = ((instr & 0xFFC00000) >> 22) as u32;
        let instruction_body = ((instr & 0x003FFFFF) >> 0) as u32;

        let instruction_ = match opcode_grp {
            0x244 => "add".to_string(),
            0x254 => "sub".to_string(),
            0x264 => "mul".to_string(),
            0x274 => "div".to_string(),
            _ => "Operation does not exist".to_string(),
        };

        (instruction_, instruction_body)
    }

}
