use std::{sync::{Arc, Mutex}, thread};

#[derive(Debug)]
struct Stack<T> {
    inner: Mutex<Vec<T>>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Self {
            inner: Mutex::new(Vec::new()),
        }
    }

    fn push(&self, value: T) {
        let mut inner = self.inner.lock().unwrap();
        inner.push(value);
    }

    fn pop(&self) -> Option<T> {
        let mut inner = self.inner.lock().unwrap();
        inner.pop()
    }
}

fn main() {
    let stack: Arc<Stack<isize>> = Arc::new(Stack::new());

    let num_threads = 10;
    let mut handles = vec![];
    let mut pop_handles = vec![];

    for i in 0..num_threads {
        let stack = Arc::clone(&stack);
        let handle = thread::spawn(move || {
            stack.push(i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Stack: {:?}", stack.inner.lock().unwrap());

    for _ in 0..num_threads {
        let stack = Arc::clone(&stack);
        let handle = thread::spawn(move || {
            stack.pop();
        });
        pop_handles.push(handle);
    }

    for handle in pop_handles {
        handle.join().unwrap();
    }

    println!("Stack: {:?}", stack.inner.lock().unwrap());
}