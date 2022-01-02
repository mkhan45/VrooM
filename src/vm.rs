use crate::value::{HeapVal, Val};

pub struct VM {
    pub code: Vec<u8>,
    pub stack: Vec<Val>,
    pub heap: Vec<HeapVal>,
    pub ip: usize,
}

#[repr(u8)]
#[derive(Debug)]
pub enum Instruction {
    PushInt,
    PushFloat,
    HeapConst,
    Pop,
    JumpEq,
    JumpNeq,
    Add,
    Dup,
    LEQ,
}

impl Instruction {
    pub fn from_byte(byte: u8) -> Self {
        unsafe { std::mem::transmute::<u8, Self>(byte) }
    }
}

impl VM {
    fn read_next_isize(&mut self) -> isize {
        let start = self.ip + 1;
        let end = self.ip + 1 + std::mem::size_of::<isize>();
        let int_bytes: [u8; 8] = unsafe {
            self.code[start..end].try_into().unwrap_unchecked()
        };

        self.ip = end;
        isize::from_be_bytes(int_bytes)
    }

    fn read_next_usize(&mut self) -> usize {
        let start = self.ip + 1;
        let end = self.ip + 1 + std::mem::size_of::<usize>();
        let int_bytes: [u8; 8] = unsafe {
            self.code[start..end].try_into().unwrap_unchecked()
        };

        self.ip = end;
        usize::from_be_bytes(int_bytes)
    }

    pub fn run(&mut self, heap_constants: &[HeapVal]) {
        while let Some(instr) = self.code.get(self.ip) {
            use Instruction::*;

            let instr = Instruction::from_byte(*instr);
            match instr {
                PushInt => {
                    let integer = self.read_next_isize();
                    self.stack.push(Val { integer });
                }
                PushFloat => todo!(),
                HeapConst => {
                    let heap_const_index = self.read_next_usize();
                    let string_val = heap_constants.get(heap_const_index).unwrap(); 

                    self.stack.push(Val { heap_ptr: self.heap.len() });
                    self.heap.push(string_val.clone());
                    self.ip += 1;
                }
                JumpEq => {
                    let index = self.read_next_usize();
                    unsafe {
                        if let Val { boolean: true } = self.stack.pop().unwrap_unchecked() {
                            self.ip = index;
                        } else {
                            self.ip += 1;
                        }
                    }
                }
                JumpNeq => {
                    let index = self.read_next_usize();
                    unsafe {
                        if let Val { boolean: false } = self.stack.pop().unwrap_unchecked() {
                            self.ip = index;
                        } else {
                            self.ip += 1;
                        }
                    }
                }
                Add => {
                    let (a, b) = unsafe { 
                        (self.stack.pop().unwrap_unchecked().integer, self.stack.pop().unwrap_unchecked().integer) 
                    };

                    self.stack.push(Val { integer: a + b });
                    self.ip += 1;
                }
                LEQ => {
                    let (a, b) = unsafe { 
                        (self.stack.pop().unwrap_unchecked().integer, self.stack.pop().unwrap_unchecked().integer) 
                    };

                    self.stack.push(Val { boolean: b <= a });
                    self.ip += 1;
                }
                Dup => {
                    let v = self.stack[self.stack.len() - 1];
                    self.stack.push(v);
                    self.ip += 1;
                }
                Pop => {
                    self.stack.pop();
                    self.ip += 1;
                }
            }
        }
    }
}
