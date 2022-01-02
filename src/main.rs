#![feature(test)]

mod value;
mod match_bench;
mod vm;

use value::{HeapVal, Val};

use crate::vm::{Instruction, VM};

fn main() {
    assert_eq!(std::mem::size_of::<Val>(), 8);
    assert_eq!(std::mem::size_of::<HeapVal>(), 32);

    let mut heap: Vec<HeapVal> = Vec::new();
    heap.push(HeapVal::Str("asdf".to_string()));

    let mut stack: Vec<Val> = Vec::new();
    stack.push(Val { integer: 42 });
    stack.push(Val { integer: 37 });
    stack.push(Val { float: 12.0 });
    stack.push(Val { heap_ptr: 0 });

    unsafe {
        dbg!(value::pop_str(&mut stack, &heap));
        dbg!(value::pop_float(&mut stack));
        dbg!(value::pop_int(&mut stack));
        dbg!(value::pop_int(&mut stack));
    }

    println!("-------------");

    let mut code = Vec::new();
    code.push(Instruction::PushInt as u8);
    code.extend(32i64.to_be_bytes());

    code.push(Instruction::PushInt as u8);
    code.extend(i64::MAX.to_be_bytes());

    code.push(Instruction::PushInt as u8);
    code.extend(i64::MIN.to_be_bytes());

    code.push(Instruction::HeapConst as u8);
    code.extend(0usize.to_be_bytes());

    let mut vm = VM {
        code,
        stack: Vec::new(),
        heap: Vec::new(),
        ip: 0,
    };
    vm.run(&vec![HeapVal::Str("asdf".to_string())]);

    unsafe {
        dbg!(value::pop_str(&mut vm.stack, &vm.heap));
        dbg!(value::pop_int(&mut vm.stack));
        dbg!(value::pop_int(&mut vm.stack));
        dbg!(value::pop_int(&mut vm.stack));
    }

    let mut code: Vec<u8> = Vec::new();


    // PushInt 0 -- [i] - byte 0
    code.push(Instruction::PushInt as u8);
    code.extend(0i64.to_be_bytes());

    // PushInt 1 -- [i, 1] - byte 9
    code.push(Instruction::PushInt as u8);
    code.extend(1i64.to_be_bytes());

    // Add -- [i + 1] - byte 18
    code.push(Instruction::Add as u8);

    // Dup       -- [i + 1, i + 1] - byte 19
    code.push(Instruction::Dup as u8);

    // PushInt 10 -- [i + 1, i + 1, 10] - byte 20
    code.push(Instruction::PushInt as u8);
    code.extend(1000i64.to_be_bytes());

    // LEQ       -- [i + 1, i + 1 <= 10] - byte 29
    code.push(Instruction::LEQ as u8);

    // JumpEq 9  -- [i] - byte 30
    code.push(Instruction::JumpEq as u8);
    code.extend(9i64.to_be_bytes());

    let mut vm = VM {
        code,
        stack: Vec::new(),
        heap: Vec::new(),
        ip: 0,
    };
    vm.run(&Vec::new());

    unsafe {
        dbg!(value::pop_int(&mut vm.stack));
    }
}
