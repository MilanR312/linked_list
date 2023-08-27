#![feature(box_patterns)]
use std::{fmt::Debug, ptr::null_mut};


pub mod node;
pub mod iterator;
use iterator::*;
use node::*;

pub struct LinkedList<T: Default>{
    head: *mut Node<T>
}
impl<T> Default for LinkedList<T> 
where T:Default{
    fn default() -> Self {
        Self {
            head: null_mut()
        }
    }
}
impl<T> LinkedList<T>
where T:Default{
    pub fn is_empty(&self) -> bool {
        self.head.is_null()
    }
    pub fn len(&self) -> usize {
        if self.is_empty(){
            return 0;
        }
        let mut root = unsafe{ & *self.head };
        let mut cnt = 1;
        while let Some(x) = root.next(){
            root = x;
            cnt += 1;
        }
        cnt
    }
    
    pub fn new_empty() -> Self {
        Self::default()
    }
    pub fn new(val: T) -> Self {
        Self {
            head: Node::new_raw(val)
        }
    }
    
    pub fn get(&self, ind: usize) -> Option<&T> {
        let node = self.get_node(ind);
        node.map(|item| &item.data)
    }
    pub fn get_node(&self, ind: usize) -> Option<&Node<T>>{
        if self.is_empty(){
            return None
        }
        let mut root = unsafe{& *self.head};
        for _ in 0..ind{
            if let Some(x) = root.next(){
                root = x;
            } else {
                return None;
            }
        }
        Some(&root)
    }
    pub fn get_mut(&mut self, ind: usize) -> Option<&mut T> {
        let node = self.get_node_mut(ind);
        node.map(|item| &mut item.data)
    }
    pub fn get_node_mut(&mut self, ind: usize) -> Option<&mut Node<T>>{
        if self.is_empty(){
            return None
        }
        let mut root = unsafe{&mut *self.head};
        for _ in 0..ind{
            if let Some(x) = root.next_mut(){
                root = x;
            } else {
                return None;
            }
        }
        Some(root)
    }
    

    ///inserts into the linked list, if > then the lengt inserts at the back
    pub fn insert(&mut self, ind: usize, val: T){
        if ind == 0 {
            self.push_front(val);
            return;
        }
        if ind > self.len() {
            self.push_back(val);
            return;
        }
        let new_node = Node::new_raw(val);

        //get the element currently at the index;
        let old_current_index = self.get_node_mut(ind).unwrap();
        
        //get element before, unwrap is safe since old current index can not be the root node
        let before_new_element = old_current_index.prev;
        
        unsafe {
            (*new_node).prev = before_new_element;
            (*new_node).next = old_current_index.get_self_mut();
            (*before_new_element).next = new_node;
        }
        old_current_index.prev = new_node;

    }
    pub fn extract(&mut self, ind: usize) -> Option<T>{
        if ind == 0{
            return self.extract_front();
        }
        if ind >= self.len() {
            return None;
        }
        let element_to_remove = self.get_node_mut(ind).unwrap();
        let element_after = element_to_remove.next;
        let element_before = element_to_remove.prev;
        unsafe {
            if !element_after.is_null(){
                (*element_after).prev = element_before;
            }
            (*element_before).next = element_after;
        }
        let out = std::mem::replace(&mut element_to_remove.data, T::default());
        unsafe {
            let b = Box::from_raw(element_to_remove.get_self_mut());
            std::mem::drop(b);
        }
        Some(out)
    }
    
    pub fn push_back(&mut self, val: T){
        unsafe{
            if self.head.is_null(){
                self.head = Node::new_raw(val);
            } else {
                (*self.head).push_back(val);
            }
        }
    }
    pub fn extract_back(&mut self) -> Option<T>{
        if self.is_empty() {
            return None;
        }
        unsafe {
            //linked list only has one element
            if (*self.head).next.is_null() {
                //there is only one element in the list so we return it
                println!("extracting root");
                let out = std::mem::replace(&mut (*self.head).data, T::default());
                let b = Box::from_raw(self.head);
                std::mem::drop(b);
                //std::ptr::drop_in_place(self.head);
                self.head = null_mut();
                Some(out)
            } else {
                println!("extracting element");
                let last_node = (*self.head).get_last_node();
                /*if last_node.prev.is_null() {
                    return None;
                }*/
                let new_last =  &mut *last_node.prev;
                new_last.next = null_mut();
                
                //last node now contains just the value and null pointers
                last_node.prev = null_mut();
                
                //get the value out
                let out = std::mem::replace(&mut last_node.data, T::default());
                //println!("dropping {:?}", last_node as *const Node<T>);
                //std::ptr::drop_in_place(last_node);
                let b = Box::from_raw(last_node);
                std::mem::drop(b);
                
                Some(out)
            }

        }
    }   
    pub fn push_front(&mut self,val: T){
        let new_head = Node::new_raw(val);
        //head is possibly null
        let old_head = self.head;
        unsafe {
            (*new_head).next = old_head;
        }
        //if the old head ist null then create a back reference
        if !old_head.is_null(){
            unsafe{
                (*old_head).prev = new_head;
            }
        }
        self.head = new_head;
    }
    pub fn extract_front(&mut self) -> Option<T> {
        if self.is_empty(){
            return None;
        }
        unsafe{
            let node_to_remove = &mut *self.head;
            let next_node = if node_to_remove.next.is_null() {
                null_mut()
            } else {
                let r = node_to_remove.next;
                (*r).prev = null_mut();
                r
            };

            //get the value out
            let out = std::mem::replace(&mut node_to_remove.data, T::default());
            //self.head = next_node;
            let old_node = std::mem::replace(&mut self.head, next_node);
            let b = Box::from_raw(old_node);
            std::mem::drop(b);

            Some(out)

        }
    }

    pub fn get_front(&self) -> Option<&T>{
        self.get(0)
    }
    pub fn get_front_mut(&mut self) -> Option<&mut T>{
        self.get_mut(0)
    }

    pub fn get_last_node(&self) -> Option<&Node<T>>{
        if self.is_empty(){
            return None;
        }
        let mut start = unsafe{& *self.head};
        while let Some(x) = start.next(){
            start = x;
        }
        Some(start)
    }
    pub fn get_last(&self) -> Option<&T>{
        let last = self.get_last_node();
        last.map(|f| &f.data)
    }
    pub fn get_last_node_mut(&mut self) -> Option<&mut Node<T>>{
        if self.is_empty(){
            return None;
        }
        let mut start = self.head;
        while !unsafe{(*start).next}.is_null(){
            start = unsafe{(*start).next};
        }
        Some(unsafe{&mut *start})
    }
    pub fn get_last_mut(&mut self) -> Option<&mut T>{
        let last = self.get_last_node_mut();
        last.map(|f| &mut f.data)
    }

}
impl<T> Drop for LinkedList<T> 
where T: Default{
    fn drop(&mut self) {
        while let Some(x) = self.extract_front() {
            std::mem::drop(x);
        }
    }
}
impl<T> Debug for LinkedList<T>
where T: Debug + Default{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LinkedList<{}>{{\n", std::any::type_name::<T>())?;
        if !self.head.is_null(){
            let mut node = unsafe{&*self.head};
            loop {
                write!(f, "{:?}", node)?;
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

    //advanced tests are for checking if the pointers are correct after operation
    #[test]
    fn basic_list(){
        let root = LinkedList::new(8);
        assert_eq!(root.get(0), Some(&8));
        let root = LinkedList::new(50);
        assert_eq!(root.get(1), None);
    }
    #[test]
    fn basic_list_advanced(){
        let root = LinkedList::new(20);
        let r = root.head;
        assert!(!r.is_null());
        let root_raw = unsafe{& (*r)};
        assert_eq!(root_raw.prev, null_mut());
        assert_eq!(root_raw.next, null_mut());
    }

    #[test]
    fn push_back(){
        let mut root = LinkedList::new(5);
        root.push_back(6);
        assert_eq!(root.get(1), Some(&6));
        assert_eq!(root.get(2), None);
    }
    #[test]
    fn push_back_advanced(){
        let mut root = LinkedList::new(5);
        root.push_back(6);
        assert!(!root.head.is_null());
        let element1 = unsafe{ &mut *root.head };
        assert!(!element1.next.is_null());
        let element2 = unsafe{&mut * element1.next};

        assert_eq!(element1.prev, null_mut());
        assert_eq!(element2.next, null_mut());

        assert_eq!(element1.next, element2.get_self_mut());
        assert_eq!(element2.prev, element1.get_self_mut());

    }

    #[test]
    fn push_front(){
        let mut root = LinkedList::new(8);
        root.push_front(5);
        assert_eq!(root.get(0), Some(&5));
        assert_eq!(root.get(1), Some(&8));
    }
    #[test]
    fn push_front_advanced(){
        let mut root = LinkedList::new(8);
        root.push_front(5);
        assert!(!root.head.is_null());
        let element1 = unsafe{&mut *root.head};
        assert!(!element1.next.is_null());
        let element2 = unsafe{&mut *element1.next};

        assert_eq!(element1.prev, null_mut());
        assert_eq!(element2.next, null_mut());

        assert_eq!(element1.next, element2.get_self_mut());
        assert_eq!(element2.prev, element1.get_self_mut());
    }

    #[test]
    fn pop_back(){
        let mut root = LinkedList::new("test".to_string());
        root.push_back("test2".to_string());

        let back = root.extract_back();
        assert_eq!(back, Some("test2".to_string()));
        let back = root.extract_back();
        assert_eq!(back, Some("test".to_string()));
        let back = root.extract_back();
        assert_eq!(back, None);
    }

    #[test]
    fn pop_back_advanced(){
        let mut root = LinkedList::new(20);
        root.push_back(2);

        let _back = root.extract_back();
        let root_raw = unsafe{&mut *root.head};
        assert_eq!(root_raw.prev, null_mut());
        assert_eq!(root_raw.next, null_mut());
    }
    #[test]
    fn pop_front(){
        let mut root = LinkedList::new("test".to_string());
        root.push_back("test2".to_string());

        let back = root.extract_front();
        assert_eq!(back, Some("test".to_string()));
        let back = root.extract_front();
        assert_eq!(back, Some("test2".to_string()));
        let back = root.extract_front();
        assert_eq!(back, None);
    }
    #[test]
    fn pop_front_advanced(){
        let mut root = LinkedList::new(20);
        root.push_back(2);

        let _back = root.extract_front();
        let root_raw = unsafe{&mut *root.head};
        assert_eq!(root_raw.prev, null_mut());
        assert_eq!(root_raw.next, null_mut());
    }

    #[test]
    fn get_node(){
        let mut root = LinkedList::new(5);
        root.push_back(8);
        let x = root.get_node(1);
        assert!(x.is_some());
        let x = x.unwrap();
        assert_eq!(x.data, 8);
        let x = x.prev();
        assert!(x.is_some());
        let x = x.unwrap();
        assert_eq!(x.data, 5);
        let x = x.prev();
        assert!(x.is_none());

    }

    #[test]
    fn len(){
        let mut root = LinkedList::new_empty();
        assert_eq!(root.len(), 0);
        root.push_back(5);
        assert_eq!(root.len(), 1);
        root.push_back(6);
        assert_eq!(root.len(), 2);
        let _ = root.extract_back();
        assert_eq!(root.len(), 1);
    }

    #[test]
    fn insert(){
        let mut root = LinkedList::new(5);
        root.push_back(7);
        root.insert(1, 20);

        assert_eq!(root.get(0), Some(&5));
        assert_eq!(root.get(1), Some(&20));
        assert_eq!(root.get(2), Some(&7));
    }
    #[test]
    fn insert_advanced(){
        let mut root = LinkedList::new(5);
        root.push_back(7);
        root.insert(1, 20);

        let elem1 = root.get_node(0).unwrap();
        let elem2 = root.get_node(1).unwrap();
        let elem3 = root.get_node(2).unwrap();

        
        assert_eq!(elem1.next as *const Node<i32>, elem2.get_self());
        assert_eq!(elem3.prev as *const Node<i32>, elem2.get_self());

        assert_eq!(elem2.prev as *const Node<i32>, elem1.get_self());
        assert_eq!(elem2.next as *const Node<i32>, elem3.get_self());
    }
    #[test]
    fn extract(){
        let mut root = LinkedList::new(5);
        root.push_back(7);
        root.push_back( 20);

        let to_remove = root.extract(1);
        assert_eq!(to_remove, Some(7));
        let to_remove = root.extract(1);
        assert_eq!(to_remove, Some(20));
        let to_remove = root.extract(1);
        assert_eq!(to_remove, None);
        
    }
    #[test]
    fn extract_advanced(){
        let mut root = LinkedList::new(5);
        root.push_back(7);
        root.push_back( 20);
        let _ = root.extract(1);

        //check if the pointers are alright now
        let elem1 = root.get_node(0).unwrap();
        let elem2 = root.get_node(1).unwrap();
        assert_eq!(elem1.next as *const Node<i32>, elem2.get_self());
        assert_eq!(elem2.prev as *const Node<i32>, elem1.get_self());

    }
}