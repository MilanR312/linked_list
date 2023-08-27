use std::{ptr::null_mut, fmt::Debug};

pub struct Node<T: Default>{
    pub data: T,
    pub(crate) next: *mut Node<T>,
    pub(crate) prev: *mut Node<T>
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
        let new_node = Self::new_raw(value);
        let last_node = self.get_last_node();
        
        last_node.next = new_node;
        unsafe { 
            (*new_node).prev = last_node;
        };
    }
    pub fn next(&self) -> Option<&Self>{
        if self.next.is_null(){
            None
        } else {
            Some( unsafe{& *self.next} )
        }
    }
    pub fn prev(&self) -> Option<&Self>{
        if self.prev.is_null(){
            None
        } else {
            Some( unsafe{ &*self.prev } )
        }
    }
    pub fn next_mut(&mut self) -> Option<&mut Self>{
        if self.next.is_null(){
            None
        } else {
            Some( unsafe{&mut *self.next} )
        }
    }
    pub fn prev_mut(&mut self) -> Option<&mut Self>{
        if self.prev.is_null(){
            None
        } else {
            Some( unsafe{ &mut *self.prev } )
        }
    }
    
    #[allow(unused)]
    pub(crate) fn get_self(&self) -> *const Self{
        self as *const Self
    }
    #[allow(unused)]
    pub(crate) fn get_self_mut(&mut self) -> *mut Self{
        self as *mut Self
    }
    pub fn new(value: T) -> Self {
        Self {
            data: value,
            next: null_mut(),
            prev: null_mut()
        }
    }
    pub(crate) fn new_raw(value: T) -> *mut Self{
        let a = Box::new(Self::new(value));
        Box::into_raw(a)
    }
}
impl <T> Drop for Node<T>
where T:Default{
    fn drop(&mut self) {
        println!("dropped single {:?}", self as *mut Node<T>);
    }
}
impl <T> Debug for Node<T>
where T: Debug + Default {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\t{{  prev={:14?}", self.prev)?;
        write!(f, "\tself={:14?}", (self as *const Node<T>))?;
        write!(f, "\tvalue={:5?}", self.data)?;
        write!(f, "\tnext={:14?} }}\n", self.next)
    }
}