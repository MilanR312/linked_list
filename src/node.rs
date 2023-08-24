use std::{alloc::{Layout, alloc}, ptr::null_mut};

pub struct Node<T: Default>{
    pub data: T,
    pub next: *mut Node<T>,
    pub prev: *mut Node<T>
}
impl<T> Node<T>
where T: Default{
    pub fn get_last_node(&mut self) -> &mut Node<T> {
        let mut node = self;
        while !node.next.is_null(){
            node = unsafe {&mut *node.next}
        }
        node
    }
    pub fn push_back(&mut self, value: T){
        let last_node = self.get_last_node();
        let layout = Layout::new::<Node<T>>();
        let new_node = unsafe { alloc(layout) as *mut Node<T> };
        
        last_node.next = new_node;
        unsafe { 
            (*new_node).prev = last_node;
            (*new_node).data = value;
        };
    }

    pub fn new(value: T) -> Self {
        Self {
            data: value,
            next: null_mut(),
            prev: null_mut()
        }
    }
}
impl <T> Drop for Node<T>
where T:Default{
    fn drop(&mut self) {
        println!("dropped single {:?}", self as *mut Node<T>);
    }
}