const INT_TYPE_ID: usize = 0;
const FLOAT_TYPE_ID: usize = 1;
const STR_TYPE_ID: usize = 2;
const BOOL_TYPE_ID: usize = 3;
const BASE_TYPE_IDS: [&str; 4] = [
    "int",
    "float",
    "str",
    "bool",
];

/// Theoretically we always know the types of things
/// so we don't need a tag
/// When we have a sum type of which we don't know the
/// tag, just use one extra slot for the type id
/// If that strategy doesn't work for sum types just put them on
/// the heap

#[derive(Copy, Clone)]
pub union Val {
    pub integer: isize,
    pub float: f64,
    pub heap_ptr: usize,
    pub boolean: bool,
    pub sum_type_id: usize,
}

#[derive(Clone)]
pub enum HeapVal {
    Str(String),
    List(Vec<Val>),
    Value(Val),
    Empty,
}

pub unsafe fn peek_is_int(stack: &[Val]) -> bool {
    stack.get_unchecked(stack.len() - 2).sum_type_id == INT_TYPE_ID
}

pub unsafe fn pop_int(stack: &mut Vec<Val>) -> isize {
    stack.pop().unwrap_unchecked().integer
}

pub unsafe fn pop_float(stack: &mut Vec<Val>) -> f64 {
    stack.pop().unwrap_unchecked().float
}

pub unsafe fn pop_bool(stack: &mut Vec<Val>) -> bool {
    stack.pop().unwrap_unchecked().boolean
}

pub unsafe fn pop_str<'a>(stack: &mut Vec<Val>, heap: &'a Vec<HeapVal>) -> &'a str {
    let i = stack.pop().unwrap_unchecked().heap_ptr;
    match &heap[i] {
        HeapVal::Str(s) => &s,
        _ => todo!(),
    }
}
