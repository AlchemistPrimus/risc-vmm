//! CPU utility module. Represents the RAM
#![allow(unused_variables)]

mod cpu_utilities {

    /// LIFO cpu stack
    #[derive(Clone, Debug)]
    pub struct Stack {
        data: Vec<u32>,
        length: u8,
    }

    impl Stack {
        /// Pushing data into the stack.
        pub fn push_stack(&mut self, data: u32){
            // Add item into the stack.
            // Throw an error if the stack is full
            if self.stack_length() == 16 {
                println!("The stack is full");
                return;
            }
            self.data.push(data);
            self.length += 1;
        }

        /// Poping data out of the stack.
        pub fn pop_stack(&mut self) -> Option<u32> {
            // Remove an item from the stack.
            // Throw an error if the stack is empty.
            if self.is_empty() {
                println!("The stack is empty");
                return None;
            }
            let data_: u32 = self.data.pop()?;
            self.length -= 1;
            Some(data_)
        }

        /// Checking if stack is empty
        pub fn is_empty(&self) -> bool {
            if self.stack_length() == 0 {
                return true;
            }
            false
        }

        /// Checking the length of the stack
        pub fn stack_length(&self) -> u8 {
            self.length
        }
    }

    /// Memory allocated on the heap
    pub struct Memory {
        data: Box<u32>,
    }

    impl Memory {
        /// Load data from the memory
        pub fn load(&mut self, _data: u32){
            println!("The memory load function");
            return;
        }

        /// Store data into the memory
        pub fn store(&mut self, _data: u32){
            println!("The memory store function");
            return;
        }
    }
}
