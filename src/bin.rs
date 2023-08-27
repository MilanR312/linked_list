
use linked_list::{*, node::Node};

fn dothing<'a>(list: &LinkedList<&'a str>){
    list.iter().for_each(|f| println!("{f}"));
}

fn main(){
    let mut root = LinkedList::new("test".to_string());
    println!("node size = {}", std::mem::size_of::<Node<String>>());
    for i in 1..10{
        let str = format!("{i}");
        root.push_back(str);
    }
    let a = root.extract(5);
    let b = root.extract_back();
    let c = root.extract_front();

    let str = "test".to_string();
    root.insert(2, str);
    for i in 1..10{
        let str = format!("{i}");
        root.push_back(str);
    }
}