
use linked_list::*;

fn dothing<'a>(list: &LinkedList<&'a str>){
    list.iter().for_each(|f| println!("{f}"));
}

fn main(){
    let mut root = LinkedList::new("test".to_string());
    for i in 1..1000{
        let str = format!("{i}");
        root.push_back(str);
    }
    let a = root.extract(5);
    let b = root.extract_back();
    let c = root.extract_front();

    let str = "test".to_string();
    root.insert(300, str);

    let ptr = Box::leak(Box::new(vec![42; 100]));
    println!("ptr={:?}", ptr)
}