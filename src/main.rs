#![feature(test)]

mod value;
mod match_bench;

use std::mem::ManuallyDrop;

use value::{HeapVal, Val};

fn main() {
    dbg!(std::mem::size_of::<Val>());
    dbg!(std::mem::size_of::<HeapVal>());

    let mut heap: Vec<HeapVal> = Vec::new();
    heap.push(HeapVal { string: ManuallyDrop::new("asdf".to_string()) });

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
}
