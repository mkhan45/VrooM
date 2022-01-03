use crate::value::{HeapVal, Val};

use std::collections::HashSet;

// 32 kb
pub const MAX_HEAP_SIZE: usize = 1000usize;

pub struct VM {
    pub code: Vec<u8>,
    pub stack: Vec<Val>,
    pub heap: Vec<HeapVal>,
    pub ip: usize,
    pub heap_top: usize,

    // hopefully this can be removed with a compiler?
    // otherwise, it's probably best to add a marked
    // flag to HeapVal.
    pub heap_roots: HashSet<usize>,
}

#[repr(u8)]
#[derive(Debug)]
pub enum Instruction {
    PushInt,
    PushFloat,
    HeapConst,
    Pop,
    PopHeapPtr,
    Jump,
    JumpEq,
    JumpNeq,
    Add,
    Dup,
    LEQ,
    Incr,
}

impl Instruction {
    pub fn from_byte(byte: u8) -> Self {
        unsafe { std::mem::transmute::<u8, Self>(byte) }
    }
}

impl VM {
    fn collect_garbage(&mut self) {
        // roots
        // right now nothing can contain a pointer so there are only roots
        let marked: HashSet<usize> = unsafe {
            self.heap_roots.iter()
                .map(|i| self.stack.get_unchecked(*i))
                .map(|v| v.heap_ptr)
                .collect()
        };

        let mut i = 0;
        let mut shift = 0;
        self.heap.retain(|v| {
            if matches!(v, &HeapVal::Empty) {
                i += 1;
                true
            } else if !marked.contains(&i) {
                shift += 1;
                i += 1;
                true
            } else {
                i += 1;
                false
            }
        });

        marked.iter().for_each(|i| unsafe {
            self.stack.get_unchecked_mut(*i).heap_ptr -= shift;
        });
        self.heap_top -= shift;
    }

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
                    // TODO: maybe CoW
                    let heap_const_index = self.read_next_usize();
                    let string_val = heap_constants.get(heap_const_index).unwrap(); 

                    if self.heap_top == MAX_HEAP_SIZE - 1 {
                        self.collect_garbage();
                    }

                    self.stack.push(Val { heap_ptr: self.heap_top });
                    self.heap_roots.insert(self.heap_top);
                    unsafe {
                        *self.heap.get_unchecked_mut(self.heap_top) = string_val.clone();
                    }
                    self.heap_top += 1;
                }
                Jump => {
                    let index = self.read_next_usize();
                    self.ip = index;
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
                Incr => {
                    unsafe { 
                        let len = self.stack.len();
                        let x = self.stack.get_unchecked_mut(len - 1);
                        x.integer += 1;
                    };
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
                    let v = unsafe { *self.stack.get(self.stack.len() - 1).unwrap_unchecked() };
                    self.stack.push(v);
                    self.ip += 1;
                }
                Pop => {
                    self.stack.pop();
                    self.ip += 1;
                }
                PopHeapPtr => {
                    let ptr = unsafe { self.stack.pop().unwrap_unchecked().heap_ptr };
                    self.heap_roots.remove(&ptr);
                    self.ip += 1;
                }
            }
        }
    }
}
