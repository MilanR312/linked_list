
use linked_list::*;

fn main(){
    let mut list = LinkedList::new(5);
    list.push_back(20);
    println!("{:?}", list);
    let x = list.extract_last();
    println!("{:?}", list);
    println!("x == {x:?}");
    list.push_back(5);
    println!("{:?}", list);
}