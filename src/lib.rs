#![feature(box_patterns)]
use std::{fmt::{Debug, Display, write}, ptr::null_mut, alloc::{Layout, alloc}, ops::Index};


pub mod node;
use node::*;

pub struct LinkedList<T: Default>{
    head: Option<Node<T>>
}
impl<T> Default for LinkedList<T> 
where T:Default{
    fn default() -> Self {
        Self {
            head: None
        }
    }
}
impl<T> LinkedList<T>
where T:Default{
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }
    pub fn new_empty() -> Self {
        Self::default()
    }
    pub fn new(val: T) -> Self {
        Self {
            head: Some(Node::new(val))
        }
    }
    pub fn get(&self, ind: usize) -> Option<&T> {
        if self.head.is_none() {
            return None;
        }
        let mut root = self.head.as_ref().unwrap();
        for _ in 1..ind {
            if root.next.is_null() {
                return None
            }
            root = unsafe { &*root.next };
        }
        Some(&root.data)
    }
    pub fn push_back(&mut self, val: T){
        match &mut self.head {
            None => self.head = Some(Node::new(val)),
            Some(x) => x.push_back(val)
        }
    }
    pub fn extract_last(&mut self) -> Option<T>{
        if self.is_empty() {
            return None;
        }
        if self.head.as_ref().unwrap().next.is_null() {
            //there is only one element in the list so we return it
            //println!("extracting root");
            let out = std::mem::replace(&mut self.head.as_mut().unwrap().data, T::default());
            //unsafe { std::ptr::drop_in_place(self.head.as_mut().unwrap()) };
            self.head = None;
            Some(out)
        } else {
            //println!("extracting element");
            let last_node = self.head.as_mut().unwrap().get_last_node();
            if last_node.prev.is_null() {
                return None;
            }
            let new_last = unsafe { &mut *last_node.prev };
            new_last.next = null_mut();
            
            //last node now contains just the value and null pointers
            last_node.prev = null_mut();
            
            //get the value out
            let out = std::mem::replace(&mut last_node.data, T::default());
            //println!("dropping {:?}", last_node as *const Node<T>);
            unsafe { std::ptr::drop_in_place(last_node) };
            Some(out)
        }
    }
}
impl<T> Drop for LinkedList<T> 
where T: Default{
    fn drop(&mut self) {
        while let Some(x) = self.extract_last() {
            std::mem::drop(x);
        }
    }
}
impl<T> Debug for LinkedList<T>
where T: Debug + Default{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LinkedList<>{{\n")?;
        if let Some(x) = &self.head {
            let mut node = x;
            loop {
                write!(f, "\t{{  prev={:14?}", node.prev)?;
                write!(f, "\tself={:14?}", (node as *const Node<T>))?;
                write!(f, "\tvalue={:5?}", node.data)?;
                write!(f, "\tnext={:14?} }}\n", node.next)?;
                if node.next.is_null(){
                    break;
                }
                node = unsafe { &*node.next };
            }
        }
        write!(f, "}}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_list(){
        let root = LinkedList::new(8);
        assert_eq!(root.get(0), Some(&8));
    }

}