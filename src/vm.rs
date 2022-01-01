use crate::value::{HeapVal, Val};

pub struct VM {
    pub code: Vec<u8>,
    pub stack: Vec<Val>,
    pub heap: Vec<HeapVal>,
    pub ip: usize,
}

#[repr(u8)]
pub enum Instruction {
    PushInt,
    PushFloat,
    HeapConst,
    Pop,
}

impl Instruction {
    pub fn from_byte(byte: u8) -> Self {
        unsafe { std::mem::transmute::<u8, Self>(byte) }
    }
}

impl VM {
    pub fn run(&mut self, heap_constants: &[HeapVal]) {
        while let Some(instr) = self.code.get(self.ip) {
            use Instruction::*;

            let instr = Instruction::from_byte(*instr);
            match instr {
                PushInt => {
                    let start = self.ip + 1;
                    let end = self.ip + 1 + std::mem::size_of::<isize>();

                    let int_bytes: [u8; 8] = unsafe {
                        self.code[start..end].try_into().unwrap_unchecked()
                    };
                    let integer = isize::from_be_bytes(int_bytes);
                    
                    self.stack.push(Val { integer });
                    self.ip = end;
                }
                PushFloat => todo!(),
                HeapConst => {
                    let start = self.ip + 1;
                    let end = self.ip + 1 + std::mem::size_of::<usize>();

                    let int_bytes: [u8; 8] = unsafe {
                        self.code[start..end].try_into().unwrap_unchecked()
                    };
                    let heap_const_index = usize::from_be_bytes(int_bytes);

                    let string_val = heap_constants.get(heap_const_index).unwrap(); 

                    self.stack.push(Val { heap_ptr: self.heap.len() });
                    self.heap.push(string_val.clone());
                    self.ip = end;
                }
                Pop => {
                    self.stack.pop();
                    self.ip += 1;
                }
            }
        }
    }
}
