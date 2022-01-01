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
    pub sum_type_id: TypeId,
}

// no point making it smaller than
// a usize since it takes up one
// stack index either way
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum TypeId {
    Int = 0,
    Float = 1,
}

#[derive(Clone)]
pub enum HeapVal {
    Str(String),
    List(Vec<Val>),
}

pub unsafe fn pop_int(stack: &mut Vec<Val>) -> isize {
    stack.pop().unwrap_unchecked().integer
}

pub unsafe fn pop_float(stack: &mut Vec<Val>) -> f64 {
    stack.pop().unwrap_unchecked().float
}

pub unsafe fn pop_str<'a>(stack: &mut Vec<Val>, heap: &'a Vec<HeapVal>) -> &'a str {
    let i = stack.pop().unwrap_unchecked().heap_ptr;
    match &heap[i] {
        HeapVal::Str(s) => &s,
        _ => todo!(),
    }
}
