//! CPU utility module. Represents the RAM
#![allow(unused_variables)]

/// LIFO cpu stack. Will be used to handle function calls.
#[derive(Debug)]
pub struct Stack {
    pub data: Vec<u32>,
    pub size: u32,
}

impl Iterator for Stack {
    type Item = u32;
    fn next(&mut self) => Options<Self::Item>{
        if !self.0.is_empty(){
            self.0.size -= 1;
            self.0.data.pop()
        } else {
            None
        }
    }
}

impl IntoIterator for Stack {
    type Item = u32;
    type IntoIter: Iterator<Item=Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Stack {
    /// Initialize the stack. Returns an empty stack
    pub fn new() -> Self {
        Self {
            size: 0,
            data: Vec::new(),
        }
    }

    /// Add an item onto the top of the stack.
    pub fn push(&mut self, elem: u32){
        // Add item into the stack.
        if self.size == 16 {
            // Handle when the stack is full
            println!("The stack is full");
            return;
        }
        self.data.push(elem);
        self.size += 1;
    }

    /// Removing top element from the stack. This valu is returned.
    pub fn pop(&mut self) -> Option<u32> {
        // Remove an item from the stack.
        if self.size == 0 {
            // Handle when the stack is empty
            println!("The stack is empty");
            return None;
        }
        self.size -= 1;
        self.data.pop()
    }

    /// Test if the stack is empty
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// Checking the length of the stack
    pub fn len(&self) -> u32{
        self.size
    }

    /// Clear the stack
    fn clear(&mut self) {
        self.size = 0;
        self.data.clear();
    }

    /// Returning reference to the top element from the stack without removing it.
    fn peek(&self) -> Option<&u32> {
        if self.size == 0 {
            return None;
        }
        self.data.get((self.size-1) as usize)
    }

    fn peek_mut(&mut self) -> Option<&mut u32>{
        if self.size == 0 {
            return None;
        }
        self.data.get_mut((self.size-1) as usize)
    }
}

/// Memory allocated on the heap.
/// Data here is allocated on the heap.
pub struct Memory {
    pub data: Vec<Box<u32>>,
}

impl Memory {
    pub fn new() -> Memory{
        Memory {
            data: Vec::new(),
        }
    }

    /// Testing function for pushing data
    pub fn push_data(&mut self, data: Box<u32>){
        self.data.push(data);
    }

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
