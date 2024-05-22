pub fn reset_stack(mut stack: Vec<String>) -> Vec<String> {
    stack.clear();
    stack.shrink_to_fit();
    return stack;
}

pub fn reset_heap(mut heap: Vec<String>) -> Vec<String> {
    heap.clear();
    heap.shrink_to_fit();
    return heap;
}

pub fn pop_stack(mut stack: Vec<String>) -> Vec<String> {
    stack.pop();
    return stack;
}

pub fn pop_heap(mut heap: Vec<String>) -> Vec<String> {
    heap.pop();
    return heap;
}


