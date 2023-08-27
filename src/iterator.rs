use std::{marker::PhantomData, ptr::{null, null_mut}};

use crate::{LinkedList, node::Node};

pub struct IntoIter<T: Default>(LinkedList<T>);

impl<T> IntoIterator for LinkedList<T>
where T:Default{
    type Item = T;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}
impl<T> Iterator for IntoIter<T>
where T:Default
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.extract_front()
    }
}
impl<T> DoubleEndedIterator for IntoIter<T>
where T:Default{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.extract_back()
    }
}


pub struct Iter<'a, T>
where T:Default
{
    next_node: *const Node<T>,
    prev_node: *const Node<T>,
    _data: PhantomData<&'a T>
}
impl<'a, T> IntoIterator for &'a LinkedList<T>
where T:Default
{
    //TODO: empty list support
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {

        Iter { 
            next_node: self.head as *const Node<T> , 
            prev_node: self.get_last_node().map(|f| f as *const Node<T>).unwrap_or(null()),
            _data: PhantomData}
        
    }
}
impl<'a, T> Iterator for Iter<'a, T>
where T:Default {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.next_node.is_null(){
            None
        } else {
            let old_node = unsafe{& *self.next_node};
            self.next_node = old_node.next as *const Node<T>;
            Some(&old_node.data)
        }
    }
}
impl<'a, T> DoubleEndedIterator for Iter<'a, T>
where T:Default
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.prev_node.is_null(){
            None
        } else {
            let old_node = unsafe{& *self.prev_node};
            self.prev_node = old_node.prev as *const Node<T>;
            Some(&old_node.data)
        }
    }
}




pub struct IterMut<'a, T>
where T:Default
{
    next_node: *mut Node<T>,
    prev_node: *mut Node<T>,
    _data: PhantomData<&'a T>
}
impl<'a, T> IntoIterator for &'a mut LinkedList<T>
where T:Default{
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        IterMut { 
            next_node: self.head, 
            prev_node: self.get_last_node_mut().map(|f| f as *mut Node<T>).unwrap_or(null_mut()),
            _data: PhantomData
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T>
where T:Default {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.next_node.is_null(){
            None
        } else {
            let old_node = unsafe{&mut *self.next_node};
            self.next_node = old_node.next;
            Some(&mut old_node.data)
        }
    }
}
impl<'a, T> DoubleEndedIterator for IterMut<'a, T>
where T: Default
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.prev_node.is_null(){
            None
        } else {
            let old_node = unsafe{&mut *self.prev_node};
            self.prev_node = old_node.prev;
            Some(&mut old_node.data)
        }
    }
}




impl<T> LinkedList<T>
where T: Default
{
    pub fn iter<'a>(&'a self) -> Iter<'a, T>{
        self.into_iter()
    }
}
impl<T> LinkedList<T>
where T:Default
{
    pub fn iter_mut<'a>(&'a mut self) -> IterMut<'a, T>{
        self.into_iter()
    }
}



#[cfg(test)]
mod iter_test{
    use crate::LinkedList;

    #[test]
    fn into_iter(){
        let mut list = LinkedList::new(1);
        list.push_back(2);
        list.push_front(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), None);
    }
    #[test]
    fn into_iter_rev(){
        let mut list = LinkedList::new(1);
        list.push_back(2);
        list.push_front(3);

        let mut iter = list.into_iter().rev();
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter(){
        let mut list = LinkedList::new(1);
        list.push_back(2);
        list.push_front(3);

        let mut iter = (&list).into_iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), None);
    }
    #[test]
    fn iter_rev(){
        let mut list = LinkedList::new(1);
        list.push_back(2);
        list.push_front(3);

        let mut iter = (&list).into_iter().rev();
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_mut(){
        let mut list = LinkedList::new(1);
        list.push_back(2);
        list.push_front(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), None);
    }
    #[test]
    fn iter_mut_reverse(){
        let mut list = LinkedList::new(1);
        list.push_back(2);
        list.push_front(3);

        let mut iter = list.iter_mut().rev();
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), None);
    }
    #[test]
    fn iter_mut_changes(){
        let mut list = LinkedList::new(1);
        list.push_back(2);
        list.push_front(3);

        for value in &mut list{
            *value *= 5;
        }

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&15));
        assert_eq!(iter.next(), Some(&5));
        assert_eq!(iter.next(), Some(&10));
        assert_eq!(iter.next(), None);
    }
}